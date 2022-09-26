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

mod l1_tor;
mod l2_base_node;
mod l2_wallet;
mod l3_miner;
mod l8_grafana;
mod l8_loki;
mod l8_promtail;
mod sync_progress;

pub use l1_tor::Tor;
pub use l2_base_node::TariBaseNode;
pub use l2_wallet::TariWallet;
pub use l3_miner::TariSha3Miner;
pub use l8_grafana::Grafana;
pub use l8_loki::Loki;
pub use l8_promtail::Promtail;

static DEFAULT_REGISTRY: &str = "quay.io/tarilabs";
static GRAFANA_REGISTRY: &str = "grafana";

static GENERAL_VOLUME: &str = "/var/tari";
static BLOCKCHAIN_VOLUME: &str = "/blockchain";
static GRAFANA_VOLUME: &str = "/grafana";

static VAR_TARI_PATH: &str = "/var/tari";
static BLOCKCHAIN_PATH: &str = "/blockchain";
// static GRAFANA_PATH: &str = "/grafana";
static GRAFANA_DEFAULTS_PATH: &str = "/usr/share/grafana/conf/defaults.ini";
static GRAFANA_PROVISION_PATH: &str = "/etc/grafana/provisioning/datasources/all.yml";
static LOKI_DEFAULTS_PATH: &str = "/usr/share/grafana/conf/defaults.ini";
static PROMTAIL_CONFIG_PATH: &str = "/etc/promtail/config.yml";
