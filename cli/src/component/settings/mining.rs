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

use log::warn;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use tari_launchpad_protocol::settings::LaunchpadSettings;

use crate::{
    component::{
        elements::block_with_title,
        widgets::{LabeledInput, Separator},
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static MINING_SETTINGS: Focus = focus_id!();
static MONERO_ADDRESS: Focus = focus_id!();
static SHA_THREADS: Focus = focus_id!();
static MONERO_URL: Focus = focus_id!();
static WALLET_PAYMENT_ADDRESS: Focus = focus_id!();

pub struct MiningSettings {
    expert_sep: Separator,
    monero_address: LabeledInput,
    monero_url: LabeledInput,
    sha_threads: LabeledInput<usize>,
    wallet_payment_address: LabeledInput,
}

impl MiningSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            monero_address: LabeledInput::new("Monero mining address", MONERO_ADDRESS),
            monero_url: LabeledInput::new("Monero node URL", MONERO_URL),
            sha_threads: LabeledInput::new_with_value("SHA3 threads", SHA_THREADS, 2),
            wallet_payment_address: LabeledInput::new("Wallet payment address", WALLET_PAYMENT_ADDRESS),
        }
    }

    pub fn check_for_updated_settings(&mut self, state: &mut AppState) {
        let mut should_write = false;
        if let Some(LaunchpadSettings { saved_settings, .. }) = &mut state.state.config.settings {
            if let Some(v) = self.wallet_payment_address.fetch_new_value() {
                saved_settings.set_wallet_payment_address(v);
                should_write = true;
            }
            if let Some(v) = self.monero_url.fetch_new_value() {
                saved_settings.set_monerod_url(v);
                should_write = true;
            }
            if let Some(addr) = self.monero_address.fetch_new_value() {
                saved_settings.set_monero_mining_address(addr);
                should_write = true;
            }
            if let Some(v) = self.sha_threads.fetch_new_value() {
                saved_settings.set_num_mining_threads(*v);
                should_write = true;
            }
        } else {
            warn!("The app state does not have a settings instance configured, so we cannot update the saved settings");
        }
        if should_write {
            state.update_settings();
        }
    }
}

impl Input for MiningSettings {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if let ComponentEvent::StateChanged = event {
            if let Some(settings) = &state.state.config.settings {
                if let Some(conf) = &settings.saved_settings.xmrig {
                    let value = conf.monero_mining_address.clone();
                    self.monero_address.set(value);
                }
                if let Some(conf) = &settings.saved_settings.mm_proxy {
                    let value = conf.monerod_url.clone();
                    self.monero_url.set(value);
                }
            }
            return None;
        }

        if state.focus_on == MINING_SETTINGS {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Down | Pass::Enter => {
                    state.focus_on(MONERO_ADDRESS);
                },
                _ => {},
            }
        } else if state.focus_on == MONERO_ADDRESS {
            let released = self.monero_address.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave if released => {
                    state.focus_on(MINING_SETTINGS);
                },
                Pass::Down if released => {
                    state.focus_on(SHA_THREADS);
                },
                _ => {
                    self.monero_address.on_event(event, state);
                },
            }
        } else if state.focus_on == SHA_THREADS {
            let released = self.sha_threads.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(MINING_SETTINGS);
                },
                Pass::Up if released => {
                    state.focus_on(MONERO_ADDRESS);
                },
                Pass::Down if released => {
                    state.focus_on(MONERO_URL);
                },
                _ => {
                    self.sha_threads.on_event(event, state);
                },
            }
        } else if state.focus_on == MONERO_URL {
            let released = self.monero_url.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(MINING_SETTINGS);
                },
                Pass::Up if released => {
                    state.focus_on(SHA_THREADS);
                },
                Pass::Down if released => {
                    state.focus_on(WALLET_PAYMENT_ADDRESS);
                },
                _ => {
                    self.monero_url.on_event(event, state);
                },
            }
        } else if state.focus_on == WALLET_PAYMENT_ADDRESS {
            let released = self.wallet_payment_address.is_released();
                match event.pass() {
                    Pass::Leave if released => {
                        state.focus_on(MINING_SETTINGS);
                    },
                    Pass::Up if released => {
                        state.focus_on(MONERO_URL);
                    },
                    Pass::Down if released => {
                        state.focus_on(MINING_SETTINGS);
                    },
                    _ => {
                        self.wallet_payment_address.on_event(event, state);
                    },
            }
        } else {
            //
        }
        self.check_for_updated_settings(state);
        None
    }
}

impl<B: Backend> Component<B> for MiningSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Mining Settings"), state.focus_on == MINING_SETTINGS);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.monero_address.draw(f, chunks[1], state);
        self.sha_threads.draw(f, chunks[2], state);
        self.monero_url.draw(f, chunks[3], state);
        self.wallet_payment_address.draw(f, chunks[4], state);
    }
}
