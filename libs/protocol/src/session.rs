use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct LaunchpadSession {
    pub all_active: bool,

    pub base_layer_active: bool,
    pub merge_layer_active: bool,
    pub monitoring_layer_active: bool,

    pub tor_active: bool,
    pub base_node_active: bool,
    pub wallet_active: bool,
    pub miner_active: bool,

    pub grafana_active: bool,
    pub loki_active: bool,
    pub promtail_active: bool,
}
