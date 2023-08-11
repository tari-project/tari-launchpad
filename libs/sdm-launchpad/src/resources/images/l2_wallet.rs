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
use tari_utilities::hex::Hex;
use tari_wallet_grpc_client::{
    grpc::{check_connectivity_response::OnlineStatus, GetConnectivityRequest, GetIdentityRequest},
    WalletGrpcClient,
};

use super::{TariBaseNode, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::resources::{
    config::{
        BaseNodeIdentity,
        ConnectionSettings,
        LaunchpadConfig,
        LaunchpadInnerEvent,
        LaunchpadProtocol,
        WalletConfig,
    },
    images::{BLOCKCHAIN_PATH, VAR_TARI_PATH},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct TariWallet {
    settings: Option<ConnectionSettings>,
    wallet: Option<WalletConfig>,
    identity: Option<BaseNodeIdentity>,
}

impl ManagedTask for TariWallet {
    fn id() -> TaskId {
        "Wallet".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id(), TariBaseNode::id()]
    }
}

impl ManagedContainer for TariWallet {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tari_wallet"
    }

    fn tag(&self) -> &str {
        "v0.49.2_20230628_e0e4ebc"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        let config = config?;
        self.settings = ConnectionSettings::try_extract(config);
        self.wallet = config.settings.as_ref().and_then(|s| s.wallet.clone());
        let session = &self.settings.as_ref()?.session;
        self.wallet.as_ref()?;
        Some(session.all_active || session.wallet_layer_active || session.wallet_active)
    }

    fn on_event(&mut self, event: LaunchpadInnerEvent) {
        match event {
            LaunchpadInnerEvent::IdentityReady(identity) => {
                self.identity = Some(identity);
            },
            LaunchpadInnerEvent::WalletIdentityReady(_) => {},
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
        if let Some(identity) = self.identity.as_ref() {
            if let Some(public_address) = identity.public_addresses.get(0) {
                let value = format!(
                    "wallet.custom_base_node={}::{}",
                    identity.public_key.to_hex(),
                    public_address,
                );
                args.set_pair("-p", value);
            }
        } else {
            panic!("BASE NODE NOT SET");
        }

        args.set("--log-config", "/var/tari/config/log4rs.yml");
        args.set("--seed-words-file", "/var/tari/config/seed_words.txt");
        args.flag("--enable-grpc");
        args.flag("-n");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            settings.add_tor(envs);
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
        envs.set("SHELL", "/bin/bash");
        envs.set("TERM", "linux");
        envs.set("APP_NAME", "wallet");
        envs.set("APP_EXEC", "tari_console_wallet");
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
            let request = GetConnectivityRequest {};
            let status_num = client.check_connectivity(request).await?.into_inner().status;
            let status = OnlineStatus::from_i32(status_num);
            let stage;
            match status {
                Some(OnlineStatus::Online) => {
                    stage = "Online";
                    self.online = true;
                },
                Some(OnlineStatus::Offline) => {
                    stage = "Offline";
                },
                Some(OnlineStatus::Connecting) => {
                    stage = "Connecting";
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
