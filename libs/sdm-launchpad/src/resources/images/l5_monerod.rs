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
    image::{Args, Envs, ManagedContainer, Networks},
};

use super::DEFAULT_REGISTRY;
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
};

#[derive(Debug, Default)]
pub struct Monerod {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for Monerod {
    fn id() -> TaskId {
        "Monerod".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id()]
    }
}

impl ManagedContainer for Monerod {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "monerod"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        self.settings = ConnectionSettings::try_extract(config?);
        let session = &self.settings.as_ref()?.session;
        Some(session.is_monerod_active())
    }

    fn args(&self, args: &mut Args) {
        args.flag("--non-interactive");
        args.flag("--restricted-rpc");
        args.set("--rpc-bind-ip", "0.0.0.0");
        args.flag("--confirm-external-bind");
        args.flag("--enable-dns-blocklist");
        args.set("--log-file", "/home/monerod/monerod.log");
        args.set("--fast-block-sync", "1");
        args.flag("--prune-blockchain");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("monerod", LocalNet::id());
    }
}
