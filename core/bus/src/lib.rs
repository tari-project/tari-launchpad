use tari_lp_protocol::app::{LpAction, LpDelta};
use tokio::sync::mpsc;

pub struct LpBusClient {
    pub action_tx: mpsc::UnboundedSender<LpAction>,
    pub delta_rx: mpsc::UnboundedReceiver<LpDelta>,
}

impl LpBusClient {
    pub fn connect() -> Self {
        let (action_tx, _action_rx) = mpsc::unbounded_channel();
        let (_delta_tx, delta_rx) = mpsc::unbounded_channel();
        Self {
            action_tx,
            delta_rx,
        }
    }
}

pub struct LpBusServer {
    pub action_rx: Option<mpsc::UnboundedReceiver<LpAction>>,
    pub delta_tx: mpsc::UnboundedSender<LpDelta>,
}
