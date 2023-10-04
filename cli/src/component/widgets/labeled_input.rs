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

use std::str::FromStr;

use anyhow::Error;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    component::{widgets::Label, Component, ComponentEvent, Frame, Input},
    state::{AppState, Focus},
};

pub enum Value<T> {
    Empty,
    Valid { value: T },
    Invalid { reason: String },
}

impl<T> Value<T> {
    pub fn is_valid(&self) -> bool {
        !matches!(self, Value::Invalid { .. })
    }
}

impl<T> Default for Value<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T, E: ToString> From<Result<T, E>> for Value<T> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(value) => Self::Valid { value },
            Err(err) => Self::Invalid {
                reason: err.to_string(),
            },
        }
    }
}

pub trait CharValidator: Send + 'static {
    fn is_valid(&self, c: char) -> bool;
}

impl<T> CharValidator for T
where
    T: Fn(char) -> bool,
    T: Send + 'static,
{
    fn is_valid(&self, c: char) -> bool {
        (self)(c)
    }
}

pub struct LabeledInput<T: FromStr = String> {
    input_mode: bool,
    label: String,
    content: String,
    focus: Focus,
    value: Value<T>,
    validator: Box<dyn CharValidator>,
}

impl<T: FromStr> LabeledInput<T> {
    pub fn new(label: impl ToString, focus: Focus) -> Self {
        Self::new_with_filter(label, focus, |_| true)
    }

    pub fn new_with_filter(label: impl ToString, focus: Focus, filter: impl CharValidator) -> Self {
        Self {
            input_mode: false,
            label: label.to_string(),
            content: String::new(),
            focus,
            value: Value::default(),
            validator: Box::new(filter),
        }
    }

    pub fn is_released(&self) -> bool {
        !self.input_mode
    }

    pub fn set(&mut self, value: T) {
        self.value = Value::Valid { value };
    }

    pub fn value(&self) -> Result<&T, Error> {
        match &self.value {
            Value::Valid { value } => Ok(value),
            Value::Invalid { reason } => Err(Error::msg(reason.to_owned())),
            Value::Empty => Err(Error::msg("Value is empty")),
        }
    }
}

impl<T> Input for LabeledInput<T>
where
    T: FromStr,
    T::Err: ToString,
{
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == self.focus {
            if let ComponentEvent::KeyEvent(key) = event {
                if self.input_mode {
                    // TODO: Show the cursor
                    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                    let alt = key.modifiers.contains(KeyModifiers::ALT);
                    match key.code {
                        KeyCode::Char('c') | KeyCode::Delete | KeyCode::Backspace if ctrl => {
                            self.content.clear();
                        },
                        KeyCode::Char(c) => {
                            if self.validator.is_valid(c) {
                                self.content.push(c);
                            }
                        },
                        KeyCode::Backspace => {
                            if alt {
                                self.content.clear();
                            } else {
                                self.content.pop();
                            }
                        },
                        KeyCode::Esc | KeyCode::Enter => {
                            self.input_mode = false;
                            self.value = self.content.parse().into();
                        },
                        _ => {},
                    }
                } else if let KeyCode::Enter = key.code {
                    self.input_mode = true;
                } else {
                    //
                }
            }
        }
        None
    }
}

impl<B: Backend, T: FromStr> Component<B> for LabeledInput<T> {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Percentage(40), Constraint::Percentage(60)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        let label = Label::new(&self.label);
        f.render_widget(label, h_chunks[0]);

        let (block_color, _cursor_color) = {
            if state.focus_on == self.focus {
                if self.input_mode {
                    (Color::Yellow, Color::White)
                } else {
                    (Color::Magenta, Color::White)
                }
            } else {
                (Color::White, Color::Reset)
            }
        };
        let text_color = {
            if self.value.is_valid() || self.input_mode {
                Color::White
            } else {
                Color::Red
            }
        };
        let block = Block::default()
            .border_style(Style::default().fg(block_color))
            .borders(Borders::ALL);
        let s: &str = self.content.as_ref();
        let text = Paragraph::new(s).block(block).style(Style::default().fg(text_color));
        f.render_widget(text, h_chunks[1]);
        // let input = self.input.widget();
        // f.render_widget(input, h_chunks[1]);
    }
}
