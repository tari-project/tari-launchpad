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

use std::sync::Arc;

use tact::{Notifier, Recipient, Task};
use tokio::sync::{broadcast, watch, watch::Ref};

use super::onboarding::{Onboarding, OnboardingAction, OnboardingDelta};

#[derive(Debug, Clone)]
pub struct Bus {
    state: Arc<watch::Sender<Onboarding>>,
    actions: broadcast::Sender<OnboardingAction>,
}

impl Bus {
    pub fn new() -> Self {
        let state = Onboarding::default();
        let (state_tx, _state_rx) = watch::channel(state);
        let (actions_tx, _actions_rx) = broadcast::channel(64);
        Self {
            state: Arc::new(state_tx),
            actions: actions_tx,
        }
    }

    pub fn state(&self) -> Ref<'_, Onboarding> {
        self.state.borrow()
    }

    pub fn send<M>(&mut self, action: M)
    where OnboardingAction: From<M> {
        self.actions.send(action.into()).ok();
    }

    pub fn update<M>(&mut self, delta: M)
    where OnboardingDelta: From<M> {
        self.state.send_modify(move |state| state.update(delta.into()));
    }

    pub fn changes<M>(&mut self, notifier: Notifier<M>) -> Task
    where M: Clone + Send + 'static {
        let mut rx = self.state.subscribe();
        Task::spawn(async move {
            while rx.changed().await.is_ok() {
                if let Err(_err) = notifier.notify() {
                    break;
                }
            }
        })
    }

    pub fn actions<M>(&mut self, recipient: Recipient<M>) -> Task
    where
        Option<M>: From<OnboardingAction>,
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
