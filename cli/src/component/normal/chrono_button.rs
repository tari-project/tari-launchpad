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
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub trait ChronoGetter {
    /// How long the mining is active.
    fn get_duration(&self, state: &AppState) -> Option<Duration>;
    fn get_label(&self, state: &AppState) -> &str;
}

/// A button with a clock.
pub struct ChronoButton<G> {
    getter: G,
}

impl<G> ChronoButton<G> {
    pub fn new(getter: G) -> Self {
        Self { getter }
    }
}

impl<G> Input for ChronoButton<G> {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend, G> Component<B> for ChronoButton<G>
where G: ChronoGetter
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Min(0)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        let block = Block::default();
        let inner_rect = block.inner(v_chunks[0]);
        f.render_widget(block, v_chunks[0]);

        let caption;
        let label = self.getter.get_label(state);
        if let Some(dur) = self.getter.get_duration(state) {
            let total = dur.as_secs();
            let seconds = total % 60;
            let total = total / 60;
            let minutes = total % 60;
            let hours = total / 60;
            caption = format!("  {:02}:{:02}:{:02} | {}  ", hours, minutes, seconds, label);
        } else {
            caption = format!("  {}  ", label);
        }

        let line = Line::from(vec![Span::styled(
            // "  Set up & start mining  ",
            // "  Start mining  ",
            caption,
            Style::default().bg(Color::Magenta),
        )]);
        let text = vec![line];
        let p = Paragraph::new(text);
        f.render_widget(p, inner_rect);
    }
}
