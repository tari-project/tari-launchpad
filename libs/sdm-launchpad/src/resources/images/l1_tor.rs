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

use async_trait::async_trait;
use regex::Regex;
use tari_launchpad_protocol::container::TaskProgress;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{
        checker::{CheckerContext, CheckerEvent, ContainerChecker},
        Args,
        Envs,
        ManagedContainer,
        Networks,
    },
};
use tor_hash_passwd::EncryptedKey;

use super::DEFAULT_REGISTRY;
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
};

#[derive(Debug, Default)]
pub struct Tor {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for Tor {
    fn id() -> TaskId {
        "Tor".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id()]
    }
}

impl ManagedContainer for Tor {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tor"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> Option<bool> {
        self.settings = ConnectionSettings::try_extract(config?);
        let session = &self.settings.as_ref()?.session;
        Some(session.all_active || session.base_layer_active || session.base_node_active)
    }

    fn checker(&mut self) -> Box<dyn ContainerChecker<LaunchpadProtocol>> {
        Box::new(Checker::new())
    }

    fn args(&self, args: &mut Args) {
        args.set_pair("--SocksPort", "0.0.0.0:9050");
        args.set_pair("--ControlPort", "0.0.0.0:9051");
        args.set_pair("--CookieAuthentication", 0);
        args.set_pair("--ClientOnly", 1);
        args.set_pair("--ClientUseIPv6", 1);
        if let Some(settings) = self.settings.as_ref() {
            let hashed = EncryptedKey::hash_password(settings.tor_password.deref());
            args.set_pair("--HashedControlPassword", hashed);
        }
        args.flag("--allow-missing-torrc");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("tor", LocalNet::id());
    }
}

struct Checker {
    re: Regex,
}

impl Checker {
    fn new() -> Self {
        let re = Regex::new(r"Bootstrapped\s+(?P<pct>\d+)%").unwrap();
        Self { re }
    }
}

#[async_trait]
impl ContainerChecker<LaunchpadProtocol> for Checker {
    // TODO: Add result here?
    async fn on_log_event(&mut self, record: &str, ctx: &mut CheckerContext<LaunchpadProtocol>) {
        if let Some(caps) = self.re.captures(record) {
            if let Some(value) = caps.name("pct") {
                if let Ok(value) = value.as_str().parse() as Result<i32, _> {
                    let progress = TaskProgress {
                        pct: value as u8,
                        stage: "Bootstrapping...".into(),
                    };
                    ctx.report(CheckerEvent::Progress(progress)).ok();
                    if value == 100 {
                        ctx.report(CheckerEvent::Ready).ok();
                    }
                }
            }
        }
    }
}
