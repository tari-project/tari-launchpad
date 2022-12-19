use std::{convert::identity, pin::Pin};

use anyhow::Error;
use tari_launchpad_protocol::{
    container::{TaskId, TaskStatus},
    launchpad::{Action, LaunchpadAction, LaunchpadState, Reaction},
    session::LaunchpadSession,
};
use tari_sdm_launchpad::LaunchpadBus;
use thiserror::Error;
use tokio::{
    select,
    time::{sleep, Duration, Sleep},
};

#[derive(Debug, Error)]
pub enum TestError {
    #[error("State does not exist")]
    NoState,
    #[error("Time elapsed")]
    TimeElapsed,
    #[error("Bus closed")]
    BusClosed,
}

pub struct TestStateInner {
    bus: LaunchpadBus,
    pub state: Option<LaunchpadState>,
    timeout: Pin<Box<Sleep>>,
}

impl TestStateInner {
    pub fn check_containers<F>(&self, ids: &[TaskId], func: F) -> bool
    where F: Fn(&TaskStatus) -> bool {
        if let Some(state) = self.state.as_ref() {
            ids.iter()
                .map(move |id| {
                    state
                        .containers
                        .get(id)
                        .map(|container| &container.status)
                        .map(&func)
                        .unwrap_or_default()
                })
                .all(identity)
        } else {
            false
        }
    }

    pub fn setup(duration: u64) -> Result<Self, Error> {
        let timeout = Box::pin(sleep(Duration::from_secs(duration)));
        let bus = LaunchpadBus::start()?;
        let action = Action::Action(LaunchpadAction::Connect);
        bus.incoming.send(action)?;
        let inner = TestStateInner {
            bus,
            state: None,
            timeout,
        };
        Ok(inner)
    }

    pub fn change_session<F>(&mut self, func: F) -> Result<(), Error>
    where F: FnOnce(&mut LaunchpadSession) {
        let state = self.state.as_ref().ok_or(TestError::NoState)?;
        let mut new_session = state.config.session.clone();
        func(&mut new_session);
        let event = LaunchpadAction::ChangeSession(new_session);
        let action = Action::Action(event);
        self.bus.incoming.send(action)?;
        Ok(())
    }

    pub async fn step(&mut self) -> Result<(), Error> {
        select! {
            _ = &mut self.timeout => {
                Err(TestError::TimeElapsed.into())
            }
            event = self.bus.outgoing.recv() => {
                if let Some(event) = event {
                    self.process_delta(event)
                } else {
                    Err(TestError::BusClosed.into())
                }
            }
        }
    }

    fn process_delta(&mut self, reaction: Reaction) -> Result<(), Error> {
        match reaction {
            Reaction::State(state) => {
                self.state = Some(state);
            },
            Reaction::Delta(delta) => {
                if let Some(state) = self.state.as_mut() {
                    state.apply(delta);
                }
            },
        }
        Ok(())
    }
}
