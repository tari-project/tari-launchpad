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

use std::collections::HashSet;

use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::line,
    text::Span,
    widgets::Widget,
    Frame,
};

use crate::{
    component::{AppState, Component},
    state::Focus,
};

pub struct Separator {
    focus_on: HashSet<Focus>,
    title: String,
}

impl Separator {
    pub fn new(title: &str, focus: impl IntoIterator<Item = Focus>) -> Self {
        Self {
            focus_on: focus.into_iter().collect(),
            title: title.into(),
        }
    }
}

impl<B: Backend> Component<B> for Separator {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let render = Render {
            focus: self.focus_on.contains(&state.focus_on),
            title: &self.title,
            line_set: line::NORMAL,
        };
        f.render_widget(render, rect);
    }
}

struct Render<'a> {
    focus: bool,
    title: &'a str,
    line_set: line::Set,
}

impl<'a> Widget for Render<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color = if self.focus { Color::Magenta } else { Color::White };
        let modifier = if self.focus { Modifier::BOLD } else { Modifier::empty() };
        let style = Style::default().fg(color).add_modifier(modifier);
        let span = Span::styled(self.title, style);
        let (col, row) = buf.set_span(area.left(), area.top(), &span, area.width);
        let start = col + 1;

        let y = row;
        for x in start..area.right() {
            buf.get_mut(x, y).set_symbol(self.line_set.horizontal); //"_"
        }
    }
}
