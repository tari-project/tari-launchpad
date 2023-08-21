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

mod containers;
mod errors;
mod logs;

use containers::ContainersScene;
use errors::ErrorsScene;
use logs::LogsScene;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use strum::{Display, EnumCount, EnumIter, FromRepr};

use crate::{
    component::{
        tabs::{AppTabs, TabGetter},
        Component,
        ComponentEvent,
        Frame,
        Input,
    },
    state::{focus, AppState, Focus},
};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum ExpertTabs {
    Performance,
    Containers,
    Logs,
    Errors,
}

impl TabGetter for ExpertTabs {
    fn focus_to(&self, _: &AppState) -> Focus {
        focus::ROOT
    }
}

pub struct ExpertScene {
    expert_tabs: AppTabs<ExpertTabs>,
    logs_scene: LogsScene,
    containers_scene: ContainersScene,
    errors_scene: ErrorsScene,
}

impl ExpertScene {
    pub fn new() -> Self {
        Self {
            expert_tabs: AppTabs::new(),
            logs_scene: LogsScene::new(),
            containers_scene: ContainersScene::new(),
            errors_scene: ErrorsScene::new(),
        }
    }
}

impl Input for ExpertScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.expert_tabs.on_event(event, state);
        match self.expert_tabs.selected() {
            ExpertTabs::Performance => {},
            ExpertTabs::Containers => {
                self.containers_scene.on_event(event, state);
            },
            ExpertTabs::Logs => {
                self.logs_scene.on_event(event, state);
            },
            ExpertTabs::Errors => {
                self.errors_scene.on_event(event, state);
            },
        }
    }
}

impl<B: Backend> Component<B> for ExpertScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.expert_tabs.draw(f, chunks[0], state);
        match self.expert_tabs.selected() {
            ExpertTabs::Performance => {},
            ExpertTabs::Containers => {
                self.containers_scene.draw(f, chunks[1], state);
            },
            ExpertTabs::Logs => {
                self.logs_scene.draw(f, chunks[1], state);
            },
            ExpertTabs::Errors => {
                self.errors_scene.draw(f, chunks[1], state);
            },
        }
    }
}
