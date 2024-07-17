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

use std::{ops::Deref, path::PathBuf};

use anyhow::{anyhow, Error};
use minotari_wallet_grpc_client::grpc::GetIdentityResponse;
use serde::Serialize;
use tari_common_types::{emoji::EmojiId, types::PublicKey};
pub use tari_launchpad_protocol::{
    config::LaunchpadConfig,
    settings::{LaunchpadSettings, TariNetwork},
};
use tari_launchpad_protocol::{node::BaseNodeIdentity, session::LaunchpadSession};
use tari_sdm::{config::ManagedProtocol, image::Envs};
use tari_utilities::ByteArray;

#[derive(Debug)]
pub struct LaunchpadProtocol;

impl ManagedProtocol for LaunchpadProtocol {
    type Config = LaunchpadConfig;
    type Inner = LaunchpadInnerEvent;
    type Outer = ();
}

#[derive(Debug, Clone)]
pub enum LaunchpadInnerEvent {
    IdentityReady(BaseNodeIdentity),
    WalletIdentityReady(WalletIdentity),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletIdentity {
    public_key: Vec<u8>,
    public_address: String,
    node_id: Vec<u8>,
    emoji_id: String,
}

impl TryFrom<GetIdentityResponse> for WalletIdentity {
    type Error = Error;

    fn try_from(value: GetIdentityResponse) -> Result<Self, Self::Error> {
        let public_key = PublicKey::from_vec(&value.public_key).map_err(|_| anyhow!("PublicKey failed to parse"))?;
        let emoji_id = EmojiId::from(&public_key).to_string();
        Ok(WalletIdentity {
            public_key: value.public_key,
            public_address: value.public_address,
            node_id: value.node_id,
            emoji_id,
        })
    }
}

// TODO: Use it as a field of the LaunchpadConfig
#[derive(Debug)]
pub struct ConnectionSettings {
    pub session: LaunchpadSession,
    pub tor_password: String,
    pub tari_network: TariNetwork,
    pub data_directory: PathBuf,
}

impl ConnectionSettings {
    pub fn try_extract(config: &LaunchpadConfig) -> Option<Self> {
        let settings = config.settings.as_ref()?;
        Some(ConnectionSettings {
            session: config.session.clone(),
            tor_password: settings.tor_control_password.clone(),
            tari_network: settings.saved_settings.tari_network,
            data_directory: settings.data_directory.clone(),
        })
    }
}

impl ConnectionSettings {
    pub fn add_tor(&self, module: &str, envs: &mut Envs) {
        let value = format!("password={}", self.tor_password.deref());
        let module = module.to_uppercase();
        let var_name = format!("TARI_{module}__P2P__TRANSPORT__TOR__CONTROL_AUTH");
        envs.set(&var_name, value);
    }

    pub fn add_common(&self, envs: &mut Envs) {
        envs.set("TARI_NETWORK", self.tari_network.lower_case());
        envs.set("DATA_FOLDER", self.data_directory.to_str().unwrap_or(""));
        envs.set("TARI_LOG_CONFIGURATION", "/var/tari/config/log4rs.yml");
        let path = concat!(
            "/usr/local/sbin:",
            "/usr/local/bin:",
            "/usr/sbin:",
            "/usr/bin:",
            "/sbin:",
            "/bin",
        );
        envs.set("PATH", path);
    }
}
