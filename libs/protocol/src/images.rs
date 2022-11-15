use serde::Serialize;
use strum::{EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, EnumIter, FromRepr, PartialEq, Eq, Hash, Serialize)]
pub enum ImageType {
    Tor,
    BaseNode,
    Wallet,
    XmRig,
    Sha3Miner,
    MmProxy,
    Monerod,
    Loki,
    Promtail,
    Grafana,
}
