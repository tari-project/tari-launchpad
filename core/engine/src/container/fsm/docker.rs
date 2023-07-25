use crate::container::{
    Config, ContainerTaskFsm, CreateContainerOptions, CreateImageOptions, DockerEvent, PullProgress,
};
use crate::types::{Args, ContainerState, Envs, Mount, Mounts, Networks, Ports, TaskId, Volumes};
use anyhow::{anyhow as err, Error};
use bollard::container::{NetworkingConfig, RemoveContainerOptions};
use bollard::image::RemoveImageOptions;
use bollard::models::{
    ContainerInspectResponse, EndpointSettings, HostConfig, Mount as BollardMount, MountTypeEnum,
    PortBinding, PortMap,
};
use bollard::system::EventsOptions;
use futures::StreamExt;
use std::collections::HashMap;
use std::path::Path;
use tact::Receiver;

impl<'a> ContainerTaskFsm<'a> {
    pub fn subscribe_to_events(&mut self) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["container".to_string()]);
        type_filter.insert(
            "container".to_string(),
            vec![self.task.container_info.container_name.clone()],
        );
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        let stream = self.task.docker.events(Some(opts)).map(DockerEvent::from);
        let recipient = self.ctx.recipient();
        let receiver = Receiver::connect(stream, recipient);
        self.task.events = Some(receiver);
    }

    pub async fn image_exists(&mut self) -> bool {
        self.task
            .docker
            .inspect_image(self.task.image())
            .await
            .is_ok()
    }

    pub async fn container_state(&mut self) -> ContainerState {
        let res = self
            .task
            .docker
            .inspect_container(self.task.container(), None)
            .await;
        // log::trace!("State of container {}: {:?}", self.inner.container_name, res);
        match res {
            Ok(ContainerInspectResponse {
                state: Some(state), ..
            }) => {
                if state.running.unwrap_or_default() {
                    ContainerState::Running
                } else {
                    ContainerState::NotRunning
                }
            }
            Ok(_) => ContainerState::NotRunning,
            Err(_) => ContainerState::NotFound,
        }
    }

    pub fn pull(&mut self) -> Receiver {
        let from_image = self.image().to_string();
        let opts = Some(CreateImageOptions {
            from_image,
            ..Default::default()
        });
        let stream = self
            .docker
            .create_image(opts, None, None)
            .map(PullProgress::from);
        Receiver::connect(stream, self.ctx.recipient())
    }

    pub async fn try_create_container(&mut self) -> Result<(), Error> {
        let mut args = Args::default();
        self.mc.args(&mut args);
        let mut ports = Ports::default();
        self.mc.ports(&mut ports);
        let mut envs = Envs::default();
        self.mc.envs(&mut envs);
        let name = self.container().to_string();
        let opts = CreateContainerOptions {
            name,
            platform: None,
        };

        let mut networks = Networks::default();
        self.mc.networks(&mut networks);
        let networks = self.networks_map(networks)?;

        let mut volumes = Volumes::default();
        self.mc.volumes(&mut volumes);
        let volumes = volumes_map(volumes.build());

        let mut mounts = Mounts::default();
        self.mc.mounts(&mut mounts);
        let mounts = self.mounts_map(mounts.build())?;
        let ports = ports.build();
        let config = Config {
            image: Some(self.image().to_string()),
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
        self.docker.create_container(Some(opts), config).await?;
        Ok(())
    }

    pub async fn try_start_container(&mut self) -> Result<(), Error> {
        self.docker
            .start_container::<String>(self.container(), None)
            .await?;
        Ok(())
    }

    pub async fn try_kill_container(&mut self) -> Result<(), Error> {
        self.docker
            .kill_container::<String>(self.container(), None)
            .await?;
        Ok(())
    }

    pub async fn try_remove_container(&mut self) -> Result<(), Error> {
        let opts = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };
        self.docker
            .remove_container(self.container(), Some(opts))
            .await?;
        Ok(())
    }

    pub async fn try_remove_image(&mut self) -> Result<(), Error> {
        let image_name = self.image();
        let opts = Some(RemoveImageOptions {
            force: true,
            ..Default::default()
        });
        self.docker.remove_image(image_name, opts, None).await?;
        Ok(())
    }

    fn networks_map(&self, networks: Networks) -> Result<NetworkingConfig<String>, Error> {
        let mut endpoints = HashMap::new();
        for (alias, resource) in networks.build() {
            let net_name = self
                .resource(&resource)
                .ok_or_else(|| {
                    err!(
                        "Network {:?} not available in resources. Check dependencies.",
                        resource
                    )
                })?
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
                    .ok_or_else(|| err!("Resource {:?} was not found.", volume))?
                    .to_string();
                let mount = BollardMount {
                    target: Some(target),
                    source: Some(resource),
                    typ: Some(MountTypeEnum::VOLUME),
                    volume_options: None,
                    ..Default::default()
                };
                Ok(mount)
            }
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
            }
        }
    }

    fn mounts_map(&self, mounts: Vec<Mount>) -> Result<Vec<BollardMount>, Error> {
        let mut result = Vec::new();
        for mount in mounts {
            result.push(self.make_mount(mount)?);
        }
        Ok(result)
    }

    pub fn resource(&self, id: &TaskId) -> Option<&str> {
        // TODO: Implement it
        // self.resources_map.get(id).map(String::as_ref)
        None
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

// FIXME: This might be replaceable by std::fs::canonicalize, but I don't have a windows machine to check
// TODO: Check `tokio::fs::canonicalize` as well
fn canonicalize<P: AsRef<Path>>(path: P) -> String {
    #[cfg(target_os = "windows")]
    let path = format!(
        "//{}",
        path.as_ref()
            .iter()
            .filter_map(|part| {
                use std::{ffi::OsStr, path};

                use regex::Regex;

                if part == OsStr::new(&path::MAIN_SEPARATOR.to_string()) {
                    None
                } else {
                    let drive = Regex::new(r"(?P<letter>[A-Za-z]):").unwrap();
                    let part = part.to_string_lossy().to_string();
                    if drive.is_match(part.as_str()) {
                        Some(drive.replace(part.as_str(), "$letter").to_lowercase())
                    } else {
                        Some(part)
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("/")
    );
    #[cfg(target_os = "macos")]
    let path = format!("/host_mnt{}", path.as_ref().to_string_lossy());
    #[cfg(target_os = "linux")]
    let path = path.as_ref().to_string_lossy().to_string();
    path
}
