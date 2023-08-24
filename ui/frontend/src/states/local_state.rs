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

use strum::{Display, EnumIter};
use tari_launchpad_protocol::container::TaskId;

use crate::widget::{SharedState, State};

pub static LOCAL_STATE: SharedState<LocalState> = SharedState::new();

#[derive(EnumIter, Display, Clone, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Normal,
    Expert,
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(EnumIter, Display, Clone, Debug, PartialEq, Eq)]
pub enum Scene {
    Mining,
    #[strum(to_string = "Base Node")]
    BaseNode,
    Wallet,
}

impl Default for Scene {
    fn default() -> Self {
        Self::BaseNode
    }
}

#[derive(Default, Debug)]
pub struct LocalState {
    pub scene: Scene,
    pub view_mode: ViewMode,
    pub show_logs_for: Option<TaskId>,
    pub expert_view: bool,
}

impl State for LocalState {
    type Delta = LocalStateDelta;

    fn apply(&mut self, delta: Self::Delta) {
        match delta {
            LocalStateDelta::SetScene(scene) => {
                self.scene = scene;
            },
            LocalStateDelta::SetViewMode(view_mode) => {
                self.view_mode = view_mode;
            },
            LocalStateDelta::ShowExpertView(flag) => {
                self.expert_view = flag;
            },
            LocalStateDelta::ShowLogs(task_id) => {
                self.show_logs_for = Some(task_id);
            },
            LocalStateDelta::HideLogs => {
                self.show_logs_for.take();
            },
        }
        log::debug!("Local state updated: {:?}", self);
    }
}

#[derive(Clone)]
pub enum LocalStateDelta {
    SetScene(Scene),
    SetViewMode(ViewMode),

    ShowExpertView(bool),

    ShowLogs(TaskId),
    HideLogs,
}
