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

mod elements;
mod expert;
mod header;
mod main_view;
mod motion;
mod normal;
mod settings;
mod tabs;
mod termination;
mod widgets;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use derive_more::From;
pub use main_view::MainView;
use ratatui::{backend::Backend, layout::Rect, Frame};
pub use termination::TerminationView;

use crate::state::AppState;

pub trait Component<B: Backend> {
    type State;

    /// A context reference a mutable to modify the frame.
    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pass {
    Up,
    Down,
    Left,
    Right,
    Leave, // mapped to Esc
    Enter,
    Space,
    Prev, // shift-Tab
    Next, // mapped to Tab
    Other,
    Tick,
    Quit,
}

#[derive(Debug, Clone, Copy, From)]
pub enum ComponentEvent {
    KeyEvent(KeyEvent),
    Tick,
    StateChanged,
}

impl ComponentEvent {
    pub fn pass(&self) -> Pass {
        match self {
            Self::KeyEvent(event) => {
                let ctrl = event.modifiers.contains(KeyModifiers::CONTROL);
                let shift = event.modifiers.contains(KeyModifiers::SHIFT);
                match event.code {
                    KeyCode::Up | KeyCode::Char('k') => Pass::Up,
                    KeyCode::Down | KeyCode::Char('j') => Pass::Down,
                    KeyCode::Left | KeyCode::Char('h') => Pass::Left,
                    KeyCode::Right | KeyCode::Char('l') => Pass::Right,
                    KeyCode::Esc => Pass::Leave,
                    KeyCode::Enter => Pass::Enter,
                    KeyCode::Char(' ') => Pass::Space,
                    KeyCode::Char('q') if ctrl => Pass::Quit,
                    KeyCode::Tab => {
                        if shift {
                            Pass::Prev
                        } else {
                            Pass::Next
                        }
                    },
                    _ => Pass::Other,
                }
            },
            Self::Tick => Pass::Tick,
            Self::StateChanged => Pass::Other,
        }
    }
}

pub trait Input {
    /// The result of a successful interaction with the input component.
    type Output;

    /// Handle an event and return the result of the interaction. If the event is not handled, return `None`.
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output>;
}
