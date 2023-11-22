// Copyright 2023. The Tari Project
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

use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct LaunchpadSession {
    pub all_active: bool,

    pub base_layer_active: bool,
    pub wallet_layer_active: bool,
    pub mining_layer_active: bool,
    pub merge_layer_active: bool,
    pub monitoring_layer_active: bool,

    pub tor_active: bool,
    pub base_node_active: bool,
    pub wallet_active: bool,
    pub miner_active: bool,

    pub mmproxy_active: bool,
    pub xmrig_active: bool,

    pub grafana_active: bool,
    pub loki_active: bool,
    pub promtail_active: bool,
}

impl LaunchpadSession {
    pub fn stop_all(&mut self) {
        *self = Self::default();
    }

    pub fn is_tor_active(&self) -> bool {
        self.all_active ||
            self.tor_active ||
            self.is_base_node_active() ||
            self.is_wallet_active() ||
            self.is_miner_active() ||
            self.is_mmproxy_active()
    }

    pub fn is_base_node_active(&self) -> bool {
        self.all_active ||
            self.base_layer_active ||
            self.base_node_active ||
            self.is_wallet_active() ||
            self.merge_layer_active
    }

    pub fn is_wallet_active(&self) -> bool {
        self.all_active ||
            self.wallet_layer_active ||
            self.wallet_active ||
            self.is_miner_active() ||
            self.merge_layer_active
    }

    /// Indicates which states signal that the SHA3x miner should be active
    pub fn is_miner_active(&self) -> bool {
        self.all_active || self.mining_layer_active || self.miner_active
    }

    pub fn is_mmproxy_active(&self) -> bool {
        self.all_active || self.merge_layer_active || self.mmproxy_active
    }

    pub fn is_xmrig_active(&self) -> bool {
        self.all_active || self.merge_layer_active || self.xmrig_active || self.is_mmproxy_active()
    }

    pub fn is_grafana_active(&self) -> bool {
        self.all_active || self.monitoring_layer_active || self.grafana_active
    }

    pub fn is_loki_active(&self) -> bool {
        self.all_active || self.monitoring_layer_active || self.loki_active
    }

    pub fn is_promtail_active(&self) -> bool {
        self.all_active || self.monitoring_layer_active || self.promtail_active
    }
}
