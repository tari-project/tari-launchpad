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
    where
        F: Fn(&TaskStatus) -> bool,
    {
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
    where
        F: FnOnce(&mut LaunchpadSession),
    {
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
            _ => {},
        }
        Ok(())
    }
}
