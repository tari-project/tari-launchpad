// Copyright 2022. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::{collections::HashMap, path::Path};

use anyhow::{anyhow, Error};
use bollard::container::StopContainerOptions;
use bollard::models::ContainerStateStatusEnum;
use bollard::{
    container::{
        Config, CreateContainerOptions, LogOutput, LogsOptions, NetworkingConfig, RemoveContainerOptions,
        Stats as BollardStats, StatsOptions,
    },
    errors::Error as BollardError,
    image::{CreateImageOptions, RemoveImageOptions},
    models::{
        CreateImageInfo, EndpointSettings, EventMessage, EventMessageTypeEnum, HostConfig, Mount as BollardMount,
        MountTypeEnum, PortBinding, PortMap,
    },
    system::EventsOptions,
};
use chrono::Local;
use futures::{StreamExt, TryStreamExt};
use tari_launchpad_protocol::container::{StatsData, TaskProgress};

use super::{ContainerState, Event, ImageTask};
use crate::{
    config::ManagedProtocol,
    forwarder::{Converter, Forwarder},
    image::{
        checker::{Logs, Stats},
        Args, Envs, Mount, Mounts, Networks, Ports, Volumes,
    },
    task::TaskContext,
    utils::TaskGuard,
};

// TODO: Methods could be moved to `TaskContext` or `ScopedDockerDriver`
// Container management
impl<C: ManagedProtocol> TaskContext<ImageTask<C>> {
    pub fn subscribe_to_events(&mut self) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["container".to_string()]);
        type_filter.insert("container".to_string(), vec![self.inner.container_name.clone()]);
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        let stream = self.driver.events(Some(opts)).map_err(Error::from);
        let sender = self.sender().get_direct().clone();
        let conv = EventConv {
            // TODO: Name is not necessary here
            name: self.inner.container_name.clone(),
        };
        let handler = Forwarder::start(stream, conv, sender);
        self.inner.events = Some(handler);
    }

    pub async fn image_exists(&mut self) -> bool {
        self.driver.inspect_image(&self.inner.image_name).await.is_ok()
    }

    pub async fn container_state(&mut self) -> ContainerState {
        let res = self.driver.inspect_container(&self.inner.container_name, None).await;
        // log::trace!("State of container {}: {:?}", self.inner.container_name, res);
        match res {
            Ok(ref response) => response.state.clone().map_or(
                {
                    log::warn!("State of container {}: {:?}", self.inner.container_name, res);
                    ContainerState::Dead
                },
                |state| {
                    state.status.map_or(
                        {
                            log::warn!("State of container {}: {:?}", self.inner.container_name, res);
                            ContainerState::Dead
                        },
                        |status| match status {
                            ContainerStateStatusEnum::EMPTY => ContainerState::Empty,
                            ContainerStateStatusEnum::CREATED => ContainerState::Created,
                            ContainerStateStatusEnum::RUNNING => ContainerState::Running,
                            ContainerStateStatusEnum::PAUSED => ContainerState::Paused,
                            ContainerStateStatusEnum::RESTARTING => ContainerState::Restarting,
                            ContainerStateStatusEnum::REMOVING => ContainerState::Removing,
                            ContainerStateStatusEnum::EXITED => ContainerState::Exited,
                            ContainerStateStatusEnum::DEAD => ContainerState::Dead,
                        },
                    )
                },
            ),
            Err(ref e) => {
                log::warn!("State of container {}: {:?} ({})", self.inner.container_name, res, e);
                ContainerState::Dead
            },
        }
    }

    pub fn pull(&mut self) -> TaskGuard<()> {
        let opts = Some(CreateImageOptions {
            from_image: self.inner.image_name.clone(),
            ..Default::default()
        });
        let stream = self.driver.create_image(opts, None, None).map_err(Error::from);
        let sender = self.sender().get_direct().clone();
        Forwarder::start(stream, ProgressConv, sender)
    }

    pub fn logs_stream(&mut self) -> Logs {
        let opts = LogsOptions::<String> {
            follow: true,
            stdout: true,
            stderr: true,
            ..Default::default()
        };
        let stream = self
            .driver
            .logs(&self.inner.container_name, Some(opts))
            .map(log_conv)
            .map_err(Error::from);
        Logs::new(stream)
    }

    pub fn stats_stream(&mut self) -> Stats {
        let opts = StatsOptions {
            stream: true,
            one_shot: false,
        };
        let stream = self
            .driver
            .stats(&self.inner.container_name, Some(opts))
            .map(stat_conv)
            .map_err(Error::from);
        Stats::new(stream)
    }

    pub async fn try_create_container(&mut self) -> Result<(), Error> {
        let mut args = Args::default();
        self.inner.image.args(&mut args);
        let mut ports = Ports::default();
        self.inner.image.ports(&mut ports);
        let mut envs = Envs::default();
        self.inner.image.envs(&mut envs);
        let opts = CreateContainerOptions {
            name: self.inner.container_name.clone(),
            platform: None,
        };

        let mut networks = Networks::default();
        self.inner.image.networks(&mut networks);
        let networks = self.networks_map(networks)?;

        let mut volumes = Volumes::default();
        self.inner.image.volumes(&mut volumes);
        let volumes = volumes_map(volumes.build());

        let mut mounts = Mounts::default();
        self.inner.image.mounts(&mut mounts);
        let mounts = self.mounts_map(mounts.build())?;
        let ports = ports.build();
        let config = Config {
            image: Some(self.inner.image_name.clone()),
            attach_stdin: Some(false),
            attach_stdout: Some(false),
            attach_stderr: Some(false),
            exposed_ports: Some(exposed_ports(&ports)),
            open_stdin: Some(true),
            stdin_once: Some(false),
            tty: Some(true),
            env: Some(envs.build()),
            volumes: Some(volumes),
            cmd: Some(args.build()),
            host_config: Some(HostConfig {
                binds: Some(vec![]),
                network_mode: Some("bridge".to_string()),
                port_bindings: Some(ports_map(&ports)),
                mounts: Some(mounts),
                ..Default::default()
            }),
            networking_config: Some(networks),
            ..Default::default()
        };
        self.driver.create_container(Some(opts), config).await?;
        Ok(())
    }

    pub async fn try_start_container(&mut self) -> Result<(), Error> {
        self.driver
            .start_container::<String>(&self.inner.container_name, None)
            .await?;
        Ok(())
    }

    pub async fn try_kill_container(&mut self) -> Result<(), Error> {
        self.driver
            .kill_container::<String>(&self.inner.container_name, None)
            .await?;
        Ok(())
    }

    pub async fn try_stop_container(&mut self, options: Option<StopContainerOptions>) -> Result<(), Error> {
        self.driver.stop_container(&self.inner.container_name, options).await?;
        Ok(())
    }

    pub async fn try_unpause_container(&mut self) -> Result<(), Error> {
        self.driver.unpause_container(&self.inner.container_name).await?;
        Ok(())
    }

    pub async fn try_remove_container(&mut self) -> Result<(), Error> {
        let opts = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };
        self.driver
            .remove_container(&self.inner.container_name, Some(opts))
            .await?;
        Ok(())
    }

    pub async fn try_remove_image(&mut self) -> Result<(), Error> {
        let image_name = self.inner.image_name.as_ref();
        let opts = Some(RemoveImageOptions {
            force: true,
            ..Default::default()
        });
        self.driver.remove_image(image_name, opts, None).await?;
        Ok(())
    }

    fn networks_map(&self, networks: Networks) -> Result<NetworkingConfig<String>, Error> {
        let mut endpoints = HashMap::new();
        for (alias, resource) in networks.build() {
            let net_name = self
                .resource(&resource)
                .ok_or_else(|| anyhow!("Network {:?} not available in resources. Check dependencies.", resource))?
                .to_string();
            let endpoint = EndpointSettings {
                aliases: Some(vec![alias]),
                ..Default::default()
            };
            endpoints.insert(net_name, endpoint);
        }
        Ok(NetworkingConfig {
            endpoints_config: endpoints,
        })
    }

    fn make_mount(&self, mount: Mount) -> Result<BollardMount, Error> {
        match mount {
            Mount::ToVolume { volume, target } => {
                let resource = self
                    .resource(&volume)
                    .ok_or_else(|| anyhow!("Resource {:?} was not found.", volume))?
                    .to_string();
                let mount = BollardMount {
                    target: Some(target),
                    source: Some(resource),
                    typ: Some(MountTypeEnum::VOLUME),
                    volume_options: None,
                    ..Default::default()
                };
                Ok(mount)
            },
            Mount::BindTo { source, target } => {
                let source = canonicalize(source);
                let mount = BollardMount {
                    target: Some(target),
                    source: Some(source),
                    typ: Some(MountTypeEnum::BIND),
                    bind_options: None,
                    ..Default::default()
                };
                Ok(mount)
            },
        }
    }

    fn mounts_map(&self, mounts: Vec<Mount>) -> Result<Vec<BollardMount>, Error> {
        let mut result = Vec::new();
        for mount in mounts {
            result.push(self.make_mount(mount)?);
        }
        Ok(result)
    }
}

type BollardMap = HashMap<String, HashMap<(), ()>>;

fn volumes_map(volumes: Vec<String>) -> BollardMap {
    let mut result = BollardMap::new();
    for volume in volumes {
        result.insert(volume, HashMap::default());
    }
    result
}

fn exposed_ports(ports: &[u16]) -> BollardMap {
    let mut result = BollardMap::new();
    for port in ports {
        result.insert(format!("{}/tcp", port), HashMap::default());
    }
    result
}

fn ports_map(ports: &[u16]) -> PortMap {
    let mut result = PortMap::new();
    for (k, _) in exposed_ports(ports) {
        let binding = vec![PortBinding {
            host_ip: Some(String::new()),
            host_port: Some(k.clone()),
        }];
        result.insert(k, Some(binding));
    }
    result
}

fn log_conv(res: Result<LogOutput, BollardError>) -> Result<String, Error> {
    if let Ok(LogOutput::Console { message }) = res {
        match std::str::from_utf8(message.as_ref()) {
            Ok(data) => {
                let message = data.to_string();
                log::trace!("Log: {}", message);
                Ok(message)
            },
            Err(err) => Err(anyhow!("Can't parse log text: {}", err)),
        }
    } else {
        Err(anyhow!("Unsupported log event: {:?}", res))
    }
}

fn stat_conv(res: Result<BollardStats, BollardError>) -> Result<StatsData, Error> {
    if let Ok(BollardStats {
        cpu_stats,
        memory_stats,
        ..
    }) = res
    {
        Ok(StatsData {
            timestamp: Local::now().naive_local(),
            system_cpu_usage: cpu_stats.system_cpu_usage.unwrap_or_default() as i64,
            cpu_usage: cpu_stats.cpu_usage.total_usage as i64,
            mem_limit: memory_stats.limit.unwrap_or_default().into(),
            mem_usage: memory_stats.usage.unwrap_or_default().into(),
        })
    } else {
        Err(anyhow!("Unsupported stats event: {:?}", res))
    }
}

struct ProgressConv;

impl Converter<CreateImageInfo, Event> for ProgressConv {
    fn convert(&self, res: Result<CreateImageInfo, Error>) -> Option<Event> {
        if let Err(err) = res {
            log::error!("Error while pulling image: {}", err);
            return Some(Event::PullingFailed(err.to_string()));
        }
        let info = res.unwrap();
        log::debug!("Created Image Info: {:?}", info);
        let details = info.progress_detail?;
        let current = details.current? * 100;
        let total = details.total?;
        let pct = current / total;
        let stage = info.status?;
        let progress = TaskProgress { pct: pct as u8, stage };
        Some(Event::PullingProgress(progress))
    }
}

struct EventConv {
    pub name: String,
}

impl Converter<EventMessage, Event> for EventConv {
    fn convert(&self, res: Result<EventMessage, Error>) -> Option<Event> {
        if let Ok(EventMessage {
            typ: Some(typ),
            action: Some(action),
            actor: Some(actor),
            ..
        }) = res
        {
            if let Some(attributes) = actor.attributes {
                if let Some(name) = attributes.get("name") {
                    // TODO: Check images as well
                    if self.name == *name {
                        // TODO: Check the name
                        if let EventMessageTypeEnum::CONTAINER = typ {
                            return action.try_into().ok();
                        }
                    } else {
                        log::error!("Message for other container {}, but expected {}", name, self.name);
                    }
                }
            }
        }
        None
    }
}

// FIXME: This might be replaceable by std::fs::canonicalize, but I don't have a windows machine to check
fn canonicalize<P: AsRef<Path>>(path: P) -> String {
    #[cfg(target_os = "windows")]
    let path = path.as_ref().to_string_lossy().to_string();
    #[cfg(target_os = "macos")]
    let path = format!("/host_mnt{}", path.as_ref().to_string_lossy());
    #[cfg(target_os = "linux")]
    let path = path.as_ref().to_string_lossy().to_string();
    path
}
