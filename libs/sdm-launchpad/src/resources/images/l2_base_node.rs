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

use anyhow::Error;
use async_trait::async_trait;
use log::debug;
use minotari_node_grpc_client::{grpc, BaseNodeGrpcClient};
use tari_launchpad_protocol::container::TaskProgress;
use tari_launchpad_protocol::settings::BaseNodeConfig;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{
        checker::{CheckerContext, CheckerEvent, ContainerChecker},
        Args, Envs, ManagedContainer, Mounts, Networks, Ports, Volumes,
    },
};

use super::{
    sync_progress::SyncProgress, Tor, BLOCKCHAIN_PATH, BLOCKCHAIN_VOLUME, DEFAULT_REGISTRY, GENERAL_VOLUME,
    VAR_TARI_PATH,
};
use crate::resources::images::sync_progress::SyncType;
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadInnerEvent, LaunchpadProtocol},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct TariBaseNode {
    settings: Option<ConnectionSettings>,
    config: BaseNodeConfig,
}

impl ManagedTask for TariBaseNode {
    fn id() -> TaskId {
        "Base Node".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id(), Tor::id()]
    }
}

impl ManagedContainer for TariBaseNode {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "minotari_node"
    }

    fn tag(&self) -> &str {
        "latest-nextnet"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        debug!("Reconfiguring base node");
        let config = config?;
        self.config = config
            .settings
            .as_ref()
            .and_then(|s| s.saved_settings.base_node.clone())
            .unwrap_or_default();
        self.settings = ConnectionSettings::try_extract(config);
        let session = &self.settings.as_ref()?.session;
        Some(session.is_base_node_active())
    }

    fn checker(&mut self) -> Box<dyn ContainerChecker<LaunchpadProtocol>> {
        Box::new(Checker::new())
    }

    fn args(&self, args: &mut Args) {
        args.set("--watch", "status");
        if !self.config.interactive {
            args.flag("-n");
        }
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            settings.add_tor("BASE_NODE", envs);
            envs.set(
                "TARI_BASE_NODE__DATA_DIR",
                format!("/blockchain/{}", settings.tari_network.lower_case()),
            );
        }
        envs.set("APP_NAME", "base_node");
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_142);
        ports.add(18_189);
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("base_node", LocalNet::id());
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
        volumes.add(BLOCKCHAIN_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Avoid using display here
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
            mounts.add_volume(SharedVolume::id(), BLOCKCHAIN_PATH);
        }
    }
}

/// A helper struct to track the progress of the initial block download.
struct Checker {
    progress: SyncProgress,
    identity_sent: bool,
    ready: bool,
}

impl Checker {
    fn new() -> Self {
        let progress = SyncProgress::new(0, 100);
        Self {
            progress,
            identity_sent: false,
            ready: false,
        }
    }
}

#[async_trait]
impl ContainerChecker<LaunchpadProtocol> for Checker {
    /// The interval hook in the base node checker is used to query the base node via gRPC for the current sync
    /// progress. The progress is then reported to the SDM via the `CheckerEvent::Progress` event.
    /// The task is reported as complete (`READY`) once the `sync_state` value from the `get_sync_progress` RPC call
    /// is `Done`.
    async fn on_interval(&mut self, ctx: &mut CheckerContext<LaunchpadProtocol>) -> Result<(), Error> {
        if self.ready {
            return Ok(());
        }
        // TODO: Keep the client
        let mut client = BaseNodeGrpcClient::connect("http://127.0.0.1:18142").await?;

        if !self.identity_sent {
            let identity = client.identify(grpc::Empty {}).await?.into_inner().try_into()?;
            let event = LaunchpadInnerEvent::IdentityReady(identity);
            ctx.notify(event)?;
            self.identity_sent = true;
        }

        let response = client.get_sync_progress(grpc::Empty {}).await?.into_inner();
        log::trace!("Sync progress: {:?}", response);
        self.progress.update(response);

        let progress = TaskProgress::from(&self.progress);
        log::trace!("Progress updated !common::progress={}", progress);

        match self.progress.sync_type {
            SyncType::Header | SyncType::Block => {
                ctx.report(CheckerEvent::Progress(progress)).ok();
            },
            SyncType::Done => {
                self.ready = true;
                ctx.report(CheckerEvent::Ready).ok();
            },
            _ => {},
        }
        Ok(())
    }
}
