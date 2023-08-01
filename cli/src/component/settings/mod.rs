mod base_node;
mod docker;
mod logs;
mod mining;
mod security;
mod wallet;

use base_node::BaseNodeSettings;
use docker::DockerSettings;
use logs::LogsSettings;
use mining::MiningSettings;
use security::SecuritySettings;
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use wallet::WalletSettings;

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
    Wallet,
    BaseNode,
    Docker,
    Logs,
    Security,
}

impl TabGetter for SettingsTabs {
    fn focus_to(&self, _: &AppState) -> Focus {
        focus::ROOT
    }
}

pub struct SettingsScene {
    settings_tabs: AppTabs<SettingsTabs>,
    mining_settings: MiningSettings,
    wallet_settings: WalletSettings,
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
            wallet_settings: WalletSettings::new(),
            base_node_settings: BaseNodeSettings::new(),
            docker_settings: DockerSettings::new(),
            logs_settings: LogsSettings::new(),
            security_settings: SecuritySettings::new(),
        }
    }
}

impl Input for SettingsScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.settings_tabs.on_event(event, state);
        match self.settings_tabs.selected() {
            SettingsTabs::Mining => {
                self.mining_settings.on_event(event, state);
            },
            SettingsTabs::Wallet => {
                self.wallet_settings.on_event(event, state);
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
            SettingsTabs::Wallet => {
                self.wallet_settings.draw(f, chunks[1], state);
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
