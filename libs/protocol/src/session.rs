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

    pub mmproxy_active: bool,
    pub monerod_active: bool,
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
        self.all_active || self.base_layer_active || self.tor_active
    }

    pub fn is_base_node_active(&self) -> bool {
        self.all_active || self.base_layer_active || self.base_node_active
    }

    pub fn is_wallet_active(&self) -> bool {
        self.all_active || self.base_layer_active || self.wallet_active
    }

    pub fn is_miner_active(&self) -> bool {
        self.all_active || self.base_layer_active || self.miner_active
    }

    pub fn is_mmproxy_active(&self) -> bool {
        self.all_active || self.merge_layer_active || self.mmproxy_active
    }

    pub fn is_monerod_active(&self) -> bool {
        self.all_active || self.merge_layer_active || self.monerod_active
    }

    pub fn is_xmrig_active(&self) -> bool {
        self.all_active || self.merge_layer_active || self.xmrig_active
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
