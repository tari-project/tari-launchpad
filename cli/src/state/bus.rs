use std::sync::Arc;

use tact::{Notifier, Recipient, Task};
use tokio::sync::{broadcast, watch, watch::Ref};

use super::launchpad::{LaunchpadAction, LaunchpadDelta, LaunchpadState};

#[derive(Debug, Clone)]
pub struct Bus {
    state: Arc<watch::Sender<LaunchpadState>>,
    actions: broadcast::Sender<LaunchpadAction>,
}

impl Bus {
    pub fn new() -> Self {
        let state = LaunchpadState::new();
        let (state_tx, _state_rx) = watch::channel(state);
        let (actions_tx, _actions_rx) = broadcast::channel(64);
        Self {
            state: Arc::new(state_tx),
            actions: actions_tx,
        }
    }

    pub fn state(&self) -> Ref<'_, LaunchpadState> {
        self.state.borrow()
    }

    pub fn send<M>(&mut self, action: M)
    where LaunchpadAction: From<M> {
        self.actions.send(action.into()).ok();
    }

    pub fn update<M>(&mut self, delta: M)
    where LaunchpadDelta: From<M> {
        self.state.send_modify(move |state| state.update(delta.into()));
    }

    pub fn changes<M>(&mut self, notifier: Notifier<M>) -> Task
    where M: Clone + Send + 'static {
        let mut rx = self.state.subscribe();
        Task::spawn(async move {
            while let Ok(_) = rx.changed().await {
                if let Err(_err) = notifier.notify() {
                    break;
                }
            }
        })
    }

    pub fn actions<M>(&mut self, recipient: Recipient<M>) -> Task
    where
        Option<M>: From<LaunchpadAction>,
        M: 'static,
    {
        let mut rx = self.actions.subscribe();
        Task::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Some(event) = msg.into() {
                    if let Err(_err) = recipient.send(event) {
                        // TODO: log error
                        break;
                    }
                }
            }
        })
    }
}
