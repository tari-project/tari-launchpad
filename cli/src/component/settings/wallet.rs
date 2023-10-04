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

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{elements::block_with_title, widgets::LabeledInput, Component, ComponentEvent, Frame, Input, Pass},
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static WALLET_SETTINGS: Focus = focus_id!();
static WALLET_ID: Focus = focus_id!();

pub struct WalletSettings {
    wallet_id: LabeledInput,
}

impl WalletSettings {
    pub fn new() -> Self {
        Self {
            wallet_id: LabeledInput::new("Tari Wallet ID (address)", WALLET_ID),
        }
    }
}

impl Input for WalletSettings {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == WALLET_SETTINGS {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Down | Pass::Enter => {
                    state.focus_on(WALLET_ID);
                },
                _ => {},
            }
        } else if state.focus_on == WALLET_ID {
            let released = self.wallet_id.is_released();
            match event.pass() {
                Pass::Up | Pass::Down | Pass::Leave if released => {
                    state.focus_on(WALLET_SETTINGS);
                },
                _ => {
                    self.wallet_id.on_event(event, state);
                },
            }
        } else {
            //
        }
        None
    }
}

impl<B: Backend> Component<B> for WalletSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Wallet Settings"), state.focus_on == WALLET_SETTINGS);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.wallet_id.draw(f, chunks[0], state);
    }
}
