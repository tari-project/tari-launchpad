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

use std::ops::Deref;

use anyhow::Error;
use async_trait::async_trait;
use log::{debug, error};
use tari_app_grpc::tari_rpc::{ConnectivityStatus, Empty};
use tari_launchpad_protocol::container::TaskProgress;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{
        checker::{CheckerContext, CheckerEvent, ContainerChecker},
        Args,
        Envs,
        ManagedContainer,
        Mounts,
        Networks,
        Ports,
        Volumes,
    },
};
use tari_wallet_grpc_client::{grpc::GetIdentityRequest, WalletGrpcClient};

use super::{TariBaseNode, Tor, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::resources::{
    config::{
        BaseNodeIdentity,
        ConnectionSettings,
        LaunchpadConfig,
        LaunchpadInnerEvent,
        LaunchpadProtocol,
        WalletConfig,
        WalletIdentity,
    },
    images::{BLOCKCHAIN_PATH, VAR_TARI_PATH},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct TariWallet {
    settings: Option<ConnectionSettings>,
    wallet: Option<WalletConfig>,
    base_node_identity: Option<BaseNodeIdentity>,
    identity: Option<WalletIdentity>,
}

impl ManagedTask for TariWallet {
    fn id() -> TaskId {
        "Wallet".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id(), Tor::id(), TariBaseNode::id()]
    }
}

impl ManagedContainer for TariWallet {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "minotari_console_wallet"
    }

    fn tag(&self) -> &str {
        "0.52"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        debug!("Reconfiguring wallet");
        let config = config?;
        self.settings = ConnectionSettings::try_extract(config);
        self.wallet = config.settings.as_ref().and_then(|s| s.saved_settings.wallet.clone());
        let session = &self.settings.as_ref()?.session;
        self.wallet.as_ref()?;
        Some(session.is_wallet_active())
    }

    fn on_event(&mut self, event: LaunchpadInnerEvent) {
        match event {
            LaunchpadInnerEvent::IdentityReady(identity) => {
                self.base_node_identity = Some(identity);
            },
            LaunchpadInnerEvent::WalletIdentityReady(identity) => {
                self.identity = Some(identity);
            },
        }
    }

    fn checker(&mut self) -> Box<dyn ContainerChecker<LaunchpadProtocol>> {
        Box::new(Checker::new())
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_143);
        ports.add(18_188);
    }

    // TODO: Add `Result`
    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
        args.set("--seed-words-file", "/var/tari/config/seed_words.txt");
        args.flag("--enable-grpc");
        // args.flag("-n");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            settings.add_tor("WALLET", envs);
            envs.set("WAIT_FOR_TOR", 0);
            envs.set(
                "TARI_BASE_NODE__DATA_DIR",
                format!("/blockchain/{}", settings.tari_network.lower_case()),
            );
        }
        if let Some(wallet) = self.wallet.as_ref() {
            // TODO: Use `.reveal()` instead
            envs.set("TARI_WALLET_PASSWORD", wallet.password.deref());
        }
        if let Some(identity) = self.base_node_identity.as_ref() {
            let connection = identity.connection_string();
            envs.set("TARI_WALLET__CUSTOM_BASE_NODE", connection);
        } else {
            error!("[Wallet config] Base node public key is unknown. Wallet will not be able to connect.");
        }
        envs.set("SHELL", "/bin/bash");
        envs.set("TERM", "linux");
        envs.set("APP_NAME", "wallet");
        envs.set("APP_EXEC", "minotari_console_wallet");
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("wallet", LocalNet::id());
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Avoid using display here
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
            mounts.add_volume(SharedVolume::id(), BLOCKCHAIN_PATH);
        }
    }
}

struct Checker {
    identity_sent: bool,
    online: bool,
}

impl Checker {
    fn new() -> Self {
        Self {
            identity_sent: false,
            online: false,
        }
    }
}

#[async_trait]
impl ContainerChecker<LaunchpadProtocol> for Checker {
    async fn on_interval(&mut self, ctx: &mut CheckerContext<LaunchpadProtocol>) -> Result<(), Error> {
        let mut client = WalletGrpcClient::connect("http://127.0.0.1:18143").await?;

        if !self.identity_sent {
            let request = GetIdentityRequest {};
            let identity = client.identify(request).await?.into_inner().try_into()?;
            let event = LaunchpadInnerEvent::WalletIdentityReady(identity);
            ctx.notify(event)?;
            self.identity_sent = true;
        }

        if !self.online {
            let status = client.get_network_status(Empty {}).await?.into_inner();
            debug!("Wallet status: {:?}", status);
            let connection_status = ConnectivityStatus::from_i32(status.status);
            let stage;
            match connection_status {
                Some(ConnectivityStatus::Online) => {
                    stage = "Online";
                    self.online = true;
                },
                Some(ConnectivityStatus::Offline) => {
                    stage = "Offline";
                },
                Some(ConnectivityStatus::Initializing) => {
                    stage = "Initializing";
                },
                Some(ConnectivityStatus::Degraded) => {
                    stage = "Degraded";
                    self.online = true;
                },
                None => {
                    stage = "Unknown status";
                },
            }
            let progress = TaskProgress {
                pct: 10,
                stage: stage.into(),
            };
            ctx.report(CheckerEvent::Progress(progress)).ok();
        }

        if self.identity_sent && self.online {
            ctx.report(CheckerEvent::Ready).ok();
        }

        Ok(())
    }
}
