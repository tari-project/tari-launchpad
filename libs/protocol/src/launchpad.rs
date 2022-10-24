// Copyright 2022. The Tari Project
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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    config::LaunchpadConfig,
    container::{TaskDelta, TaskId, TaskState},
    wallet::{WalletDelta, WalletState},
};

/// An action sent from UI to the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Action(LaunchpadAction),
    Start,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchpadAction {
    Connect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchpadDelta {
    UpdateConfig(LaunchpadConfig),
    SetActive(bool),
    TaskDelta(TaskId, TaskDelta),
    WalletDelta(WalletDelta),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LaunchpadState {
    pub config: Option<LaunchpadConfig>,
    pub containers: HashMap<TaskId, TaskState>,
    pub wallet: WalletState,
}

impl LaunchpadState {
    pub fn new() -> Self {
        Self {
            config: None,
            containers: HashMap::new(),
            wallet: WalletState::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reaction {
    State(LaunchpadState),
    Delta(LaunchpadDelta),
}

impl LaunchpadState {
    pub fn apply(&mut self, delta: LaunchpadDelta) {
        use LaunchpadDelta::*;
        match delta {
            // TODO: Change to UpdateSettings
            UpdateConfig(config) => {
                self.config = Some(config);
            },
            // UpdateSession(session) => {
            // if let Some(config) =
            // },
            SetActive(flag) => {
                if let Some(config) = self.config.as_mut() {
                    config.session.active = flag;
                }
            },
            TaskDelta(task_id, delta) => {
                self.containers.entry(task_id).or_default().apply(delta);
            },
            WalletDelta(delta) => {
                self.wallet.apply(delta);
            },
        }
    }
}
