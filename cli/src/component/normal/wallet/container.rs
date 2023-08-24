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

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Padding, Paragraph},
};

use crate::{
    component::{
        elements::{block_with_title, logo},
        normal::wallet::{BALANCE, SEND_FUNDS},
        widgets::{ChronoButton, ChronoGetter},
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    focus_id,
    state::{focus, AppState, Focus},
};

const LOGO: &str = r#"
╦ ╦┌─┐┬  ┬  ┌─┐┌┬┐
║║║├─┤│  │  ├┤  │
╚╩╝┴ ┴┴─┘┴─┘└─┘ ┴
"#;

pub static WALLET_CONTAINER: Focus = focus_id!();
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
}

impl Input for WalletContainerWidget {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == WALLET_CONTAINER {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Right | Pass::Next => {
                    state.focus_on(BALANCE);
                },
                Pass::Down => {
                    state.focus_on(SEND_FUNDS);
                },
                Pass::Enter | Pass::Space => {
                    let session = &mut state.state.config.session;
                    session.wallet_active = !session.wallet_active;
                    state.update_state();
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
        let block =
            block_with_title(Some("Wallet"), state.focus_on == WALLET_CONTAINER).padding(Padding::new(1, 1, 1, 0));
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(3),
            // Constraint::Percentage(50),
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        // self.status_badge.draw(f, v_chunks[0], state);

        let logo = logo(LOGO);
        f.render_widget(logo, v_chunks[0]);

        let mut lines = Vec::new();
        if let Some(wallet_id) = state.state.wallet.wallet_id.as_ref() {
            lines.push(make_line("EmojiId", &wallet_id.emoji_id));
            lines.push(make_line("Tari Address", &wallet_id.tari_address));
        }
        let p = Paragraph::new(lines);
        f.render_widget(p, v_chunks[1]);

        // self.tari_amount.draw(f, v_chunks[3], state);

        self.button.draw(f, v_chunks[3], state);
    }
}

fn make_line<'a>(title: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": "),
        Span::raw(value),
    ])
}
