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

mod base_node;
mod docker;
mod logs;
mod mining;
mod security;

use base_node::BaseNodeSettings;
use docker::DockerSettings;
use logs::LogsSettings;
use mining::MiningSettings;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use security::SecuritySettings;
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
pub enum SettingsTabs {
    Mining,
    BaseNode,
    Docker,
    Logs,
    Security,
}

impl TabGetter for SettingsTabs {
    fn focus_to(&self, _: &AppState) -> Focus {
        match self {
            Self::Mining => mining::MINING_SETTINGS,
            Self::BaseNode => base_node::BASE_NODE_SETTINGS,
            Self::Docker => docker::DOCKER_SETTINGS,
            Self::Logs => logs::LOGS_SETTINGS,
            _ => focus::ROOT,
        }
    }
}

pub struct SettingsScene {
    settings_tabs: AppTabs<SettingsTabs>,
    mining_settings: MiningSettings,
    base_node_settings: BaseNodeSettings,
    docker_settings: DockerSettings,
    logs_settings: LogsSettings,
    security_settings: SecuritySettings,
}

impl SettingsScene {
    pub fn new() -> Self {
        Self {
            settings_tabs: AppTabs::new(),
            mining_settings: MiningSettings::new(),
            base_node_settings: BaseNodeSettings::new(),
            docker_settings: DockerSettings::new(),
            logs_settings: LogsSettings::new(),
            security_settings: SecuritySettings::new(),
        }
    }
}

impl Input for SettingsScene {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        self.settings_tabs.on_event(event, state);
        match self.settings_tabs.selected() {
            SettingsTabs::Mining => {
                self.mining_settings.on_event(event, state);
            },
            SettingsTabs::BaseNode => {
                self.base_node_settings.on_event(event, state);
            },
            SettingsTabs::Docker => {
                self.docker_settings.on_event(event, state);
            },
            SettingsTabs::Logs => {
                self.logs_settings.on_event(event, state);
            },
            SettingsTabs::Security => {
                self.security_settings.on_event(event, state);
            },
        }
        None
    }
}

impl<B: Backend> Component<B> for SettingsScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.settings_tabs.draw(f, chunks[0], state);
        match self.settings_tabs.selected() {
            SettingsTabs::Mining => {
                self.mining_settings.draw(f, chunks[1], state);
            },
            SettingsTabs::BaseNode => {
                self.base_node_settings.draw(f, chunks[1], state);
            },
            SettingsTabs::Docker => {
                self.docker_settings.draw(f, chunks[1], state);
            },
            SettingsTabs::Logs => {
                self.logs_settings.draw(f, chunks[1], state);
            },
            SettingsTabs::Security => {
                self.security_settings.draw(f, chunks[1], state);
            },
        }
    }
}
