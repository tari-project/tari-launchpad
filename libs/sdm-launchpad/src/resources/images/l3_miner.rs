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

use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, Envs, ManagedContainer, Mounts, Networks, Volumes},
};

use super::{TariBaseNode, TariWallet, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    images::VAR_TARI_PATH,
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct TariSha3Miner {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for TariSha3Miner {
    fn id() -> TaskId {
        "Sha3Miner".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![TariBaseNode::id(), TariWallet::id(), LocalNet::id(), SharedVolume::id()]
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
        "0.52"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        self.settings = ConnectionSettings::try_extract(config?);
        let session = &self.settings.as_ref()?.session;
        Some(session.is_miner_active())
    }

    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            settings.add_tor(envs);
            envs.set("TARI_MINER__NUM_MINING_THREADS", 8); // TODO: Get config num
            envs.set("TARI_MINER__MINE_ON_TIP_ONLY", 1);
            envs.set(
                &format!(
                    "TARI_BASE_NODE__{}__GRPC_BASE_NODE_GRPC_ADDRESS",
                    settings.tari_network.upper_case()
                ),
                "/dns4/base_node/tcp/18142",
            );
            envs.set("TARI_WALLET__GRPC_ADDRESS", "/dns4/wallet/tcp/18143");
        }
        envs.set("SHELL", "/bin/bash");
        envs.set("TERM", "linux");
        envs.set("APP_NAME", "sha3_miner");
        envs.set("APP_EXEC", "tari_miner");
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
