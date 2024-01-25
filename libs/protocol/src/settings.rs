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

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tari_common_types::tari_address::TariAddress;
use thiserror::Error;

pub const DEFAULT_MONEROD_URL: &str = "http://stagenet.xmr-tw.org:38081,\
http://stagenet.community.xmr.to:38081,\
http://monero-stagenet.exan.tech:38081,\
http://xmr-lux.boldsuck.org:38081,\
http://singapore.node.xmr.pm:38081";

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct BaseNodeConfig {
    /// Should node be started in interactive mode.
    pub interactive: bool,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct XmRigConfig {
    /// The address that will accept Monero mining rewards
    pub monero_mining_address: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Sha3MinerConfig {
    /// The number of threads to employ for SHA3 mining
    pub num_mining_threads: usize,
    pub wallet_payment_address: Option<TariAddress>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MmProxyConfig {
    /// A URL specifying the Monero daemon to connect to
    pub monerod_url: String,
    /// If required, the monero username for the monero daemon
    pub monero_username: String,
    /// If required, the password needed to access the monero deamon
    // #[serde(skip_serializing)]
    pub monero_password: String,
    /// If true, provide the monero username and password to the daemon. Otherwise those strings are ignored.
    pub monero_use_auth: bool,

    pub wallet_payment_address: Option<TariAddress>,
}

impl Default for MmProxyConfig {
    fn default() -> Self {
        MmProxyConfig {
            monerod_url: DEFAULT_MONEROD_URL.to_string(),
            monero_username: String::new(),
            monero_password: String::new(),
            monero_use_auth: false,
            wallet_payment_address: None,
        }
    }
}

impl MmProxyConfig {
    pub fn monero_use_auth(&self) -> usize {
        if self.monero_use_auth {
            1
        } else {
            0
        }
    }
}

/// Tari Launchpad configuration struct. This will generally be populated from some front-end or persistent storage
/// file and is used to generate the environment variables needed to configure and run the various docker containers.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PersistentSettings {
    /// The Tari network to use. Default = stagenet
    pub tari_network: TariNetwork,
    /// Base node configuration.
    pub base_node: Option<BaseNodeConfig>,
    /// SHA3x miner settings
    pub sha3_miner: Option<Sha3MinerConfig>,
    /// Merge-mining proxy settings
    pub mm_proxy: Option<MmProxyConfig>,
    /// XMRig settings
    pub xmrig: Option<XmRigConfig>,
    /// The Docker registry to use to download images. By default we use ghcr.io/tari-project
    pub registry: Option<String>,
    /// The docker tag to use. By default, we use 'latest'
    pub tag: Option<String>,
}

impl PersistentSettings {
    pub fn new_base_node_settings(&mut self) {
        self.base_node = Some(BaseNodeConfig::default());
    }

    pub fn new_sha3_miner_settings(&mut self) {
        self.sha3_miner = Some(Sha3MinerConfig::default());
    }

    pub fn new_mm_proxy_settings(&mut self) {
        self.mm_proxy = Some(MmProxyConfig::default());
    }

    pub fn new_xmrig_settings(&mut self) {
        self.xmrig = Some(XmRigConfig::default());
    }

    pub fn set_monero_mining_address<S: Into<String>>(&mut self, address: S) {
        if self.xmrig.is_none() {
            self.new_xmrig_settings();
        }
        if let Some(x) = self.xmrig.as_mut() {
            x.monero_mining_address = address.into()
        }
    }

    pub fn set_num_mining_threads(&mut self, num_threads: usize) {
        if self.sha3_miner.is_none() {
            self.new_sha3_miner_settings();
        }
        if let Some(s) = self.sha3_miner.as_mut() {
            s.num_mining_threads = num_threads
        }
    }

    pub fn set_monerod_url<S: Into<String>>(&mut self, url: S) {
        if self.mm_proxy.is_none() {
            self.new_mm_proxy_settings();
        }
        if let Some(m) = self.mm_proxy.as_mut() {
            m.monerod_url = url.into()
        }
    }

    pub fn set_wallet_payment_address<S: Into<String>>(&mut self, payment_address: S) {
        let address = TariAddress::from_hex(&payment_address.into()).ok();

        if self.sha3_miner.is_none() {
            self.new_sha3_miner_settings();
        }
        if let Some(s) = self.sha3_miner.as_mut() {
            s.wallet_payment_address = address.clone()
        }

        if self.mm_proxy.is_none() {
            self.new_mm_proxy_settings();
        }
        if let Some(m) = self.mm_proxy.as_mut() {
            m.wallet_payment_address = address
        }
    }
}

impl TryFrom<&str> for PersistentSettings {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value).map_err(|e| e.to_string())
    }
}

/// Tari Launchpad configuration struct. This will generally be populated from some front-end or persistent storage
/// file and is used to generate the environment variables needed to configure and run the various docker containers.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LaunchpadSettings {
    /// The directory to use for config, id files and logs
    pub data_directory: PathBuf,
    /// The tor control password to share among containers.
    pub tor_control_password: String,
    pub with_monitoring: bool,
    pub with_tor: bool,
    pub saved_settings: PersistentSettings,
}

impl Default for LaunchpadSettings {
    fn default() -> Self {
        Self {
            data_directory: PathBuf::default(),
            tor_control_password: String::new(),
            with_monitoring: true,
            with_tor: true,
            saved_settings: PersistentSettings::default(),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unsupported network: {0}")]
pub struct UnsupportedNetwork(String);

/// Supported networks for the launchpad
#[derive(Serialize, Debug, Deserialize, Clone, Copy)]
pub enum TariNetwork {
    Igor,
    Nextnet,
    Stagenet,
    Mainnet,
}

impl TariNetwork {
    pub fn lower_case(self) -> &'static str {
        match self {
            Self::Igor => "igor",
            Self::Nextnet => "nextnet",
            Self::Stagenet => "stagenet",
            Self::Mainnet => "mainnet",
        }
    }

    pub fn upper_case(self) -> &'static str {
        match self {
            Self::Igor => "IGOR",
            Self::Nextnet => "NEXTNET",
            Self::Stagenet => "STAGENET",
            Self::Mainnet => "MAINNET",
        }
    }
}

/// Default network is Esme. This will change after mainnet launch
impl Default for TariNetwork {
    fn default() -> Self {
        Self::Nextnet
    }
}

impl TryFrom<&str> for TariNetwork {
    type Error = UnsupportedNetwork;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "igor" => Ok(TariNetwork::Igor),
            "nextnet" => Ok(TariNetwork::Nextnet),
            "mainnet" => Ok(TariNetwork::Mainnet),
            other => Err(UnsupportedNetwork(other.to_owned())),
        }
    }
}
