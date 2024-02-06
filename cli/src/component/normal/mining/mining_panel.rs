// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use crossterm::event::KeyCode;
use log::warn;
use ratatui::prelude::*;
use tari_launchpad_protocol::settings::LaunchpadSettings;

use crate::component::widgets::popup::Popup;
use crate::{
    component::{
        elements::block_with_title,
        normal::mining::{
            helpers::{MergeMiningStatus, ShaMiningStatus},
            status_badge::StatusBadge,
        },
        widgets::LabeledInput,
        Component, ComponentEvent,
        ComponentEvent::KeyEvent,
        Input, Pass,
    },
    focus_id,
    state::{focus, AppState, Focus},
};

static MONERO_ADDRESS: Focus = focus_id!();
static WALLET_PAYMENT_ADDRESS: Focus = focus_id!();

pub struct MiningPanel {
    mm_status: StatusBadge<MergeMiningStatus>,
    monero_address: LabeledInput,
    sha3_status: StatusBadge<ShaMiningStatus>,
    wallet_payment_address: LabeledInput,
    show_popup: bool,
}

impl MiningPanel {
    pub fn new() -> Self {
        Self {
            mm_status: StatusBadge::new(MergeMiningStatus),
            monero_address: LabeledInput::new("Monero mining address", MONERO_ADDRESS),
            sha3_status: StatusBadge::new(ShaMiningStatus),
            wallet_payment_address: LabeledInput::new("Wallet payment address", WALLET_PAYMENT_ADDRESS),
            show_popup: false,
        }
    }

    fn toggle_merge_mining(state: &mut AppState) {
        let session = &mut state.state.config.session;
        session.merge_layer_active = !session.merge_layer_active;
        state.update_state();
    }

    fn toggle_sha3_mining(state: &mut AppState) {
        let session = &mut state.state.config.session;
        session.sha3x_layer_active = !session.sha3x_layer_active;
        state.update_state();
    }

    pub fn check_for_updated_settings(&mut self, state: &mut AppState) {
        let mut should_write = false;
        if let Some(LaunchpadSettings { saved_settings, .. }) = &mut state.state.config.settings {
            if let Some(v) = self.wallet_payment_address.fetch_new_value() {
                saved_settings.set_wallet_payment_address(v);
                should_write = true;
            }
            if let Some(addr) = self.monero_address.fetch_new_value() {
                saved_settings.set_monero_mining_address(addr);
                should_write = true;
            }
        } else {
            warn!("The app state does not have a settings instance configured, so we cannot update the saved settings");
        }
        if should_write {
            self.show_popup = false;
            state.update_settings();
        }
    }
}

impl Input for MiningPanel {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if let ComponentEvent::StateChanged = event {
            if let Some(settings) = &state.state.config.settings {
                if let Some(conf) = &settings.saved_settings.xmrig {
                    let value = conf.monero_mining_address.clone();
                    self.monero_address.set(value);
                }
                if let Some(conf) = &settings.saved_settings.sha3_miner {
                    if let Some(wallet_payment_address) = conf.wallet_payment_address.clone() {
                        self.wallet_payment_address.set(wallet_payment_address.to_string());
                    }
                }
            }
            return None;
        }

        if state.focus_on == focus::BASE_NODE {
            match event.pass() {
                Pass::Down | Pass::Enter => {
                    state.focus_on(MONERO_ADDRESS);
                },
                _ => {},
            }
        } else if state.focus_on == MONERO_ADDRESS {
            let released = self.monero_address.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave if released => {
                    state.focus_on(focus::BASE_NODE);
                },
                Pass::Down if released => {
                    state.focus_on(WALLET_PAYMENT_ADDRESS);
                },
                _ => {
                    self.monero_address.on_event(event, state);
                },
            }
        } else if state.focus_on == WALLET_PAYMENT_ADDRESS {
            let released = self.wallet_payment_address.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(focus::BASE_NODE);
                },
                Pass::Up if released => {
                    state.focus_on(MONERO_ADDRESS);
                },
                _ => {
                    self.wallet_payment_address.on_event(event, state);
                },
            }
        } else {
            // Nadda
        }

        self.check_for_updated_settings(state);

        if let KeyEvent(key) = event {
            if key.code == KeyCode::Char('m') || key.code == KeyCode::Char('M') {
                if let Some(settings) = &state.state.config.settings {
                    if let Some(conf) = &settings.saved_settings.sha3_miner {
                        if conf.wallet_payment_address.is_none() {
                            self.show_popup = true;
                            state.update_state();
                            return None;
                        }
                        if let Some(conf) = &settings.saved_settings.xmrig {
                            let value = conf.monero_mining_address.clone();
                            if value.is_empty() {
                                self.show_popup = true;
                                state.update_state();
                                return None;
                            }
                        }
                    }
                }

                Self::toggle_merge_mining(state);
                return Some(());
            }
            if key.code == KeyCode::Char('t') || key.code == KeyCode::Char('T') {
                if let Some(settings) = &state.state.config.settings {
                    if let Some(conf) = &settings.saved_settings.sha3_miner {
                        if conf.wallet_payment_address.is_none() {
                            self.show_popup = true;
                            state.update_state();
                            return None;
                        }
                    }
                }
                Self::toggle_sha3_mining(state);
                return Some(());
            }
        }

        None
    }
}

impl<B: Backend> Component<B> for MiningPanel {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(
            Some("Mining"),
            state.state.config.session.is_sha3x_active() || state.state.config.session.is_xmrig_active(),
        );
        let inner_rect = block.inner(rect);

        let v_constraints = [
            Constraint::Length(3), // Monero address
            Constraint::Length(3), // Wallet Payment address
            Constraint::Max(1),    // stretch
            Constraint::Length(1), // Merged mining status
            Constraint::Length(1), // SHA3x mining status
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(v_constraints)
            .split(inner_rect);

        f.render_widget(block, rect);
        self.monero_address.draw(f, v_chunks[0], state);
        self.wallet_payment_address.draw(f, v_chunks[1], state);
        self.mm_status.draw(f, v_chunks[2], state);
        self.sha3_status.draw(f, v_chunks[3], state);
        if self.show_popup {
            let popup = Popup::default()
                .content("You need to enter wallet information before you can start mining.")
                .style(Style::new().yellow())
                .title("Missing wallet information")
                .title_style(Style::new().white().bold())
                .border_style(Style::new().red());
            f.render_widget(popup, v_chunks[4]);
        }
    }
}
