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

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::{focus, AppState},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Expert,
    Settings,
}

/// A selector to switch between `Normal`, `Expert`, and `Settings`.
pub struct ModeSelector {
    mode: Mode,
}

impl ModeSelector {
    pub fn new() -> Self {
        Self { mode: Mode::Normal }
    }

    pub fn selected(&self) -> Mode {
        self.mode.clone()
    }
}

impl Input for ModeSelector {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if let ComponentEvent::KeyEvent(key) = event {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('n') => {
                        self.mode = Mode::Normal;
                        state.focus_on(focus::BASE_NODE);
                    },
                    KeyCode::Char('e') => {
                        self.mode = Mode::Expert;
                        state.focus_on(focus::ROOT);
                    },
                    KeyCode::Char('s') => {
                        self.mode = Mode::Settings;
                        state.focus_on(focus::ROOT);
                    },
                    _ => {},
                }
            }
        }
        None
    }
}

impl<B: Backend> Component<B> for ModeSelector {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let not_selected = Style::default().fg(Color::White);
        let selected = Style::default().fg(Color::Magenta);
        let style_for = |mode: Mode| -> Style {
            if mode == self.selected() {
                selected
            } else {
                not_selected
            }
        };
        let line = Line::from(vec![
            Span::styled("(N)ormal", style_for(Mode::Normal)),
            Span::raw(" | "),
            Span::styled("(E)xpert", style_for(Mode::Expert)),
            Span::raw(" | "),
            Span::styled("(S)ettings", style_for(Mode::Settings)),
        ]);
        let text = vec![line];
        let paragraph = Paragraph::new(text).alignment(Alignment::Right);
        f.render_widget(paragraph, rect);
    }
}
