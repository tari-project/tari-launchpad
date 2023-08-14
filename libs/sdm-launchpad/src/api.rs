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

use anyhow::Error;
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction, LaunchpadState, Reaction};
use tokio::sync::{mpsc, RwLock};

use crate::bus::LaunchpadBus;

#[allow(unused)]
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
            let mut state = self.state.write().await;
            match reaction {
                Reaction::State(new_state) => {
                    *state = new_state;
                },
                Reaction::Delta(delta) => {
                    state.apply(delta);
                },
            }
        }
    }
}
