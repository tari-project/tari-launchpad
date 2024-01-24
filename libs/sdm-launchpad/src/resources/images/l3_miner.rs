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

use log::{info, warn};
use tari_common_types::tari_address::TariAddress;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, Envs, ManagedContainer, Mounts, Networks, Volumes},
};

use super::{TariBaseNode, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    images::VAR_TARI_PATH,
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug)]
pub struct TariSha3Miner {
    settings: Option<ConnectionSettings>,
    wallet_payment_address: Option<TariAddress>,
}

impl Default for TariSha3Miner {
    fn default() -> Self {
        TariSha3Miner {
            settings: None,
            wallet_payment_address: None,
        }
    }
}

impl ManagedTask for TariSha3Miner {
    fn id() -> TaskId {
        "Sha3Miner".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![TariBaseNode::id(), LocalNet::id(), SharedVolume::id()]
    }
}

impl ManagedContainer for TariSha3Miner {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "minotari_sha3_miner"
    }

    fn tag(&self) -> &str {
        "latest-nextnet"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        self.settings = ConnectionSettings::try_extract(config?);
        let session = &self.settings.as_ref()?.session;

        self.wallet_payment_address = match config?.settings {
            Some(ref settings) if settings.saved_settings.sha3_miner.is_none() => {
                info!("No Sha3 Miner settings found for the container configuration. Falling back on defaults.");
                None
            }
            Some(ref settings) => settings.saved_settings.sha3_miner.clone()?.wallet_payment_address,
            None => {
                warn!("The settings configuration for the Sha3 Miner config is empty");
                None
            }
        };

        Some(self.wallet_payment_address.is_some() && session.is_sha3x_active())
    }

    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            envs.set("TARI_MINER__NUM_MINING_THREADS", 8); // TODO: Get config num
            envs.set("TARI_MINER__MINE_ON_TIP_ONLY", 1);
            envs.set(
                &format!(
                    "TARI_BASE_NODE__{}__GRPC_BASE_NODE_GRPC_ADDRESS",
                    settings.tari_network.upper_case()
                ),
                "/dns4/base_node/tcp/18142",
            );
        }
        envs.set("SHELL", "/bin/bash");
        envs.set("TERM", "linux");
        envs.set("APP_NAME", "minotari_sha3_miner");
        envs.set("APP_EXEC", "minotari_miner");

        if let Some(payment_address) = self.wallet_payment_address.as_ref() {
            envs.set("TARI_MINER__WALLET_PAYMENT_ADDRESS", payment_address.to_hex());
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("tari_sha3_miner", LocalNet::id());
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        if let Some(settings) = self.settings.as_ref() {
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
        }
    }
}
