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

use tari_launchpad_protocol::settings::MmProxyConfig;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, Envs, ManagedContainer, Networks},
};

use super::DEFAULT_REGISTRY;
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
};

#[derive(Debug, Default)]
pub struct MmProxy {
    settings: Option<ConnectionSettings>,
    mm_proxy: Option<MmProxyConfig>,
}

impl ManagedTask for MmProxy {
    fn id() -> TaskId {
        "MM proxy".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id()]
    }
}

impl ManagedContainer for MmProxy {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "minotari_merge_mining_proxy"
    }

    fn tag(&self) -> &str {
        "0.52"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        self.settings = ConnectionSettings::try_extract(config?);
        let session = &self.settings.as_ref()?.session;
        self.mm_proxy = config?.settings.as_ref()?.mm_proxy.clone();
        self.mm_proxy.as_ref()?; // To check it exists
        Some(session.is_mmproxy_active())
    }

    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_tor(envs);
        }
        envs.set("APP_NAME", "mm_proxy");
        envs.set("APP_EXEC", "minotari_merge_mining_proxy");
        if let Some(config) = self.mm_proxy.as_ref() {
            envs.set("TARI_MERGE_MINING_PROXY__MONEROD_URL", &config.monerod_url);
            envs.set("TARI_MERGE_MINING_PROXY__MONEROD_USERNAME", &config.monero_username);
            envs.set(
                "TARI_MERGE_MINING_PROXY__MONEROD_PASSWORD",
                config.monero_password.deref(),
            );
            envs.set("TARI_MERGE_MINING_PROXY__MONEROD_USE_AUTH", config.monero_use_auth());
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("tari_mm_proxy", LocalNet::id());
    }
}
