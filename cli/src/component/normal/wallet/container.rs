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

use std::time::Duration;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Padding, Paragraph},
};

use crate::{
    component::{
        elements::block_with_title,
        widgets::{ChronoButton, ChronoGetter},
        Component,
        ComponentEvent,
        ComponentEvent::KeyEvent,
        Frame,
        Input,
        Pass,
    },
    focus_id,
    state::{focus, AppState, Focus},
};

static BUTTON: Focus = focus_id!();

struct WalletContainerGetter;

impl ChronoGetter for WalletContainerGetter {
    fn get_duration(&self, _state: &AppState) -> Option<Duration> {
        None
    }

    fn get_label(&self, state: &AppState) -> &str {
        if state.state.config.session.is_wallet_active() {
            "Pause"
        } else {
            "Start wallet"
        }
    }
}

pub struct WalletContainerWidget {
    button: ChronoButton<WalletContainerGetter>,
}

impl WalletContainerWidget {
    pub fn new() -> Self {
        Self {
            button: ChronoButton::new(WalletContainerGetter, BUTTON),
        }
    }

    fn toggle_wallet(state: &mut AppState) {
        let session = &mut state.state.config.session;
        session.wallet_active = !session.wallet_active;
        state.update_state();
    }
}

impl Input for WalletContainerWidget {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if let KeyEvent(key) = event {
            if key.code == KeyCode::Char('w') && key.modifiers.contains(KeyModifiers::CONTROL) {
                Self::toggle_wallet(state);
                return Some(());
            }
        }
        if state.focus_on == focus::WALLET {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::BASE_NODE);
                },
                Pass::Enter | Pass::Space => {
                    Self::toggle_wallet(state);
                },
                _ => {},
            }
        }
        None
    }
}

impl<B: Backend> Component<B> for WalletContainerWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Wallet [Ctrl-W]"), state.focus_on == focus::WALLET)
            .padding(Padding::new(1, 1, 1, 0));
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Min(30),    // Stretch
            Constraint::Length(18), // Button
        ];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner_rect);
        // self.status_badge.draw(f, v_chunks[0], state);

        let mut lines = Vec::new();
        if let Some(wallet_id) = state.state.wallet.wallet_id.as_ref() {
            lines.push(make_line("EmojiId", &wallet_id.emoji_id));
            lines.push(make_line("Tari Address", &wallet_id.tari_address));
        }
        let p = Paragraph::new(lines);
        f.render_widget(p, h_chunks[0]);
        self.button.draw(f, h_chunks[1], state);
    }
}

fn make_line<'a>(title: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": "),
        Span::raw(value),
    ])
}
