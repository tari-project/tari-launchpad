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

use std::borrow::Cow;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Row, Table},
};
use tari_launchpad_protocol::{container::TaskId, errors::ErrorRecord};

use crate::{
    component::{elements::block_with_title, expert::Focus, AppState, Component, ComponentEvent, Frame, Input, Pass},
    focus_id,
    state::focus,
};

pub static ERRORS_TABLE: Focus = focus_id!();

pub struct ErrorsScene {}

impl ErrorsScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for ErrorsScene {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == ERRORS_TABLE {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                _ => {},
            }
        }
        None
    }
}

impl<B: Backend> Component<B> for ErrorsScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Errors"), state.focus_on == ERRORS_TABLE);
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(rect);
        let records = flatten_records(state);
        let mut rows = Vec::new();
        for (task_id, record) in records {
            let dt = format!("{}\n{}", record.datetime.time(), record.datetime.date());
            let items = vec![
                Cow::Owned(dt),
                Cow::Borrowed(task_id.as_ref()),
                Cow::Borrowed(record.message.as_ref()),
            ];
            let row = Row::new(items).height(3);
            rows.push(row);
        }
        let header_cells = ["DateTime", "Task", "Error"];
        let header = Row::new(header_cells)
            .style(Style::default().fg(Color::Yellow))
            .height(1)
            .bottom_margin(1);
        let table = Table::new(rows)
            .block(block)
            .header(header)
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ])
            .column_spacing(2);
        f.render_widget(table, rects[0]);
    }
}

fn flatten_records(state: &AppState) -> Vec<(&TaskId, &ErrorRecord)> {
    let mut records: Vec<_> = state
        .state
        .containers
        .iter()
        .flat_map(|(task_id, task_state)| task_state.fails.iter().rev().map(move |record| (task_id, record)))
        .collect();
    records.sort_by(|l, r| r.1.datetime.cmp(&l.1.datetime));
    records
}
