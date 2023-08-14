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
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::{focus, AppState},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Expert,
    Settings,
}

/// A selector to switch between `Normal`, `Expert`, and `Settings`.
pub struct ModeSelector {
    expert: bool,
    settings: bool,
}

impl ModeSelector {
    pub fn new() -> Self {
        Self {
            expert: false,
            settings: false,
        }
    }

    pub fn selected(&self) -> Mode {
        match (self.expert, self.settings) {
            (_, true) => Mode::Settings,
            (true, false) => Mode::Expert,
            (false, false) => Mode::Normal,
        }
    }
}

impl Input for ModeSelector {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        let mut changed = false;
        if let ComponentEvent::KeyEvent(key) = event {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('n') => {
                        self.expert = false;
                        self.settings = false;
                        changed = true;
                    },
                    KeyCode::Char('e') => {
                        self.expert = !self.expert;
                        self.settings = false;
                        changed = true;
                    },
                    KeyCode::Char('s') => {
                        self.settings = !self.settings;
                        changed = true;
                    },
                    _ => {},
                }
            }
        }
        if changed {
            state.focus_on(focus::ROOT);
        }
    }
}

impl<B: Backend> Component<B> for ModeSelector {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let not_selected = Style::default().fg(Color::White);
        let selected = Style::default().fg(Color::Magenta);
        let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
        let style_for = |mode: Mode| -> Style {
            if mode == self.selected() {
                selected
            } else {
                not_selected
            }
        };
        let selector = if self.expert { " o" } else { "o " };
        let line = Line::from(vec![
            Span::styled("Normal", style_for(Mode::Normal)),
            Span::raw(" ("),
            Span::styled(selector, bold),
            Span::raw(") "),
            Span::styled("Expert", style_for(Mode::Expert)),
            Span::raw(" | "),
            Span::styled("Settings", style_for(Mode::Settings)),
        ]);
        let text = vec![line];
        let paragraph = Paragraph::new(text).alignment(Alignment::Right);
        f.render_widget(paragraph, rect);
    }
}
