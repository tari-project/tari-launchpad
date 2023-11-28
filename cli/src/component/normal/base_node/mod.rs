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

mod container;

use container::BaseNodeWidget;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{
        Component,
        ComponentEvent,
        Frame,
        Input,
    },
    state::AppState,
};
use crate::component::widgets::status_line::{StatusLine, StatusReportGetter};

struct BaseNodeHint;

impl StatusReportGetter for BaseNodeHint {
    fn get_hint(&self, state: &AppState) -> String {
        if state.state.config.session.is_base_node_active() {
            "Running".into()
        } else {
            "Not running".into()
        }
    }
}

pub struct BaseNodeScene {
    hint: StatusLine<BaseNodeHint>,
    base_node: BaseNodeWidget,
}

impl BaseNodeScene {
    pub fn new() -> Self {
        Self {
            hint: StatusLine::new(BaseNodeHint),
            base_node: BaseNodeWidget::new(),
        }
    }
}

impl Input for BaseNodeScene {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        self.base_node.on_event(event, state)
    }
}

impl<B: Backend> Component<B> for BaseNodeScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [
            Constraint::Length(1), // hint
            Constraint::Length(7), // base node box
            Constraint::Min(0),    // extra space
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.hint.draw(f, v_chunks[0], state);

        self.base_node.draw(f, v_chunks[1], state);
    }
}
