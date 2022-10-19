use std::sync::Arc;

use anyhow::Error;
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction, LaunchpadState, Reaction};
use tokio::sync::{mpsc, RwLock};

use crate::bus::LaunchpadBus;

pub struct SdmApi {
    state: Arc<RwLock<LaunchpadState>>,
    incoming: mpsc::UnboundedSender<Action>,
}

impl SdmApi {
    pub fn install() -> Result<Self, Error> {
        let bus = LaunchpadBus::start()?;
        let state = LaunchpadState::default();
        let state = Arc::new(RwLock::new(state));
        let worker = SdmWorker {
            state: state.clone(),
            incoming: bus.incoming.clone(),
            outgoing: bus.outgoing,
        };
        tauri::async_runtime::spawn(worker.entrypoint());
        Ok(Self {
            state,
            incoming: bus.incoming,
        })
    }
}

pub struct SdmWorker {
    state: Arc<RwLock<LaunchpadState>>,
    incoming: mpsc::UnboundedSender<Action>,
    outgoing: mpsc::UnboundedReceiver<Reaction>,
}

impl SdmWorker {
    async fn entrypoint(mut self) {
        let action = Action::Action(LaunchpadAction::Connect);
        self.incoming.send(action).ok();
        while let Some(reaction) = self.outgoing.recv().await {
            self.state.write().await.apply(reaction);
        }
    }
}
