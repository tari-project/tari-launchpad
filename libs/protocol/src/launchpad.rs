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
    errors::ErrorRecord,
    frame::Frame,
    node::{NodeDelta, NodeState},
    session::LaunchpadSession,
    settings::{LaunchpadSettings, PersistentSettings},
};

/// An action sent from UI to the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Action(LaunchpadAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchpadAction {
    Connect,
    ChangeSession(LaunchpadSession),
    SaveSettings(PersistentSettings),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchpadDelta {
    UpdateConfig(LaunchpadSettings),
    UpdateSession(LaunchpadSession),
    TaskAdded { id: TaskId, state: TaskState },
    TaskDelta { id: TaskId, delta: TaskDelta },
    NodeDelta(NodeDelta),
    AddError(ErrorRecord),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchpadState {
    pub config: LaunchpadConfig,
    pub containers: HashMap<TaskId, TaskState>,
    pub node: NodeState,
    pub errors: Frame<ErrorRecord>,
}

impl Default for LaunchpadState {
    fn default() -> Self {
        Self {
            config: LaunchpadConfig::default(),
            containers: HashMap::new(),
            node: NodeState::default(),
            errors: Frame::new(30),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reaction {
    State(LaunchpadState),
    Delta(LaunchpadDelta),
}

impl LaunchpadState {
    pub fn apply(&mut self, delta: LaunchpadDelta) {
        use LaunchpadDelta::*;
        match delta {
            // TODO: Rename to UpdateSettings
            UpdateConfig(settings) => {
                self.config.settings = Some(settings);
            },
            UpdateSession(session) => {
                self.config.session = session;
            },
            TaskAdded { id, state } => {
                self.containers.insert(id, state);
            },
            TaskDelta { id, delta } => {
                if let Some(state) = self.containers.get_mut(&id) {
                    state.apply(delta);
                }
            },
            AddError(error) => {
                self.errors.push(error);
            },
            NodeDelta(delta) => {
                self.node.apply(delta);
            },
        }
    }
}
