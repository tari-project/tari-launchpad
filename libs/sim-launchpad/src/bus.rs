use anyhow::Error;
use tari_launchpad_protocol::launchpad::{Action, Reaction};
use tokio::sync::mpsc;

use crate::simulator::Simulator;

// TODO: Move (join) this `Bus` with the main sdm bus
pub struct LaunchpadBus {
    pub incoming: mpsc::UnboundedSender<Action>,
    pub outgoing: mpsc::UnboundedReceiver<Reaction>,
}

impl LaunchpadBus {
    pub fn start() -> Result<Self, Error> {
        let (in_tx, in_rx) = mpsc::unbounded_channel();
        let (out_tx, out_rx) = mpsc::unbounded_channel();
        std::thread::spawn(move || Simulator::create_and_run(in_rx, out_tx));
        Ok(Self {
            incoming: in_tx,
            outgoing: out_rx,
        })
    }
}
