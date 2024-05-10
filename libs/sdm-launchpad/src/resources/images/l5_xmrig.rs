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

use tari_launchpad_protocol::settings::XmRigConfig;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, Envs, ManagedContainer, Networks, Volumes},
};

use super::{MmProxy, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
};

#[derive(Debug, Default)]
pub struct XMRig {
    settings: Option<ConnectionSettings>,
    xmrig: Option<XmRigConfig>,
}

impl ManagedTask for XMRig {
    fn id() -> TaskId {
        "Xmrig".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), MmProxy::id()]
    }
}

impl ManagedContainer for XMRig {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "xmrig"
    }

    fn tag(&self) -> &str {
        "latest-nextnet"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        self.settings = ConnectionSettings::try_extract(config?);
        let session = &self.settings.as_ref()?.session;

        self.xmrig.clone_from(&config?.settings.as_ref()?.saved_settings.xmrig);
        self.xmrig.as_ref()?;

        Some(session.is_xmrig_active())
    }

    fn args(&self, args: &mut Args) {
        args.set("--config", "/dev/null");
        args.set("--url", "tari_mm_proxy:18081");
        args.set("--user", "${TARI_MONERO_WALLET_ADDRESS}");
        args.set("--coin", "monero");
        args.flag("--daemon");
        args.set("--log-file", "/home/tari/xmrig.log");
        args.flag("--verbose");
        // No value or a value of 0 will let XMRig use auto config, otherwise the provided value will be used
        if let Some(xmrig) = self.xmrig.as_ref() {
            if let Some(val) = xmrig.num_mining_threads.clone() {
                if let Some(num_mining_threads) = val.0 {
                    if num_mining_threads > 0 {
                        args.set("--threads", "${TARI_RANDOM_X_NUM_MINING_THREADS}");
                    }
                }
            }
        }
        args.set("--asm", "auto");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
        }
        if let Some(xmrig) = self.xmrig.as_ref() {
            envs.set("TARI_MONERO_WALLET_ADDRESS", &xmrig.monero_mining_address);

            if let Some(val) = xmrig.num_mining_threads.clone() {
                if let Some(num_mining_threads) = val.0 {
                    if num_mining_threads > 0 {
                        envs.set("TARI_RANDOM_X_NUM_MINING_THREADS", num_mining_threads);
                    }
                }
            }
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("xmrig", LocalNet::id());
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }
}
