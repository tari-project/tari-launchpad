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

use std::{borrow::Cow, cell::RefCell, collections::VecDeque};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Row, Table, TableState},
};

use crate::{
    component::{elements::block_with_title, AppState, Component, ComponentEvent, Frame, Input, Pass},
    state::focus,
};

pub struct LogsScene {
    // The cell is used, because stateful widgets must be mutable on rendering,
    // but the `Component` uses immutable reference to avoid changing data on rendering.
    table_state: RefCell<TableState>,
}

impl LogsScene {
    pub fn new() -> Self {
        Self {
            table_state: RefCell::new(TableState::default()),
        }
    }
}

impl Input for LogsScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == focus::LOGS_TABLE {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                _ => {},
            }
        }
    }
}

impl<B: Backend> Component<B> for LogsScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Logs"), state.focus_on == focus::LOGS_TABLE);
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(rect);

        let mut records: Vec<_> = state
            .state
            .containers
            .iter()
            .map(|(task_id, task_state)| task_state.tail.iter().rev().map(move |record| (task_id, record)))
            .flatten()
            .collect();
        records.sort_by(|l, r| r.1.datetime.cmp(&l.1.datetime));

        let mut rows = Vec::new();
        for (task_id, record) in records {
            let dt = format!("{}\n{}", record.datetime.time(), record.datetime.date());
            let (left, right) = split_half(&record.message);
            let message = format!("{left}\n{right}");
            let items = vec![
                Cow::Owned(dt),
                Cow::Borrowed(task_id.as_ref()),
                Cow::Borrowed(record.level.as_ref()),
                Cow::Owned(message),
            ];
            let row = Row::new(items).height(3);
            rows.push(row);
        }
        let header_cells = ["DateTime", "Localisation", "Level", "Message"];
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
                Constraint::Percentage(10),
                Constraint::Percentage(70),
            ])
            .column_spacing(2);
        f.render_stateful_widget(table, rects[0], &mut *self.table_state.borrow_mut());
    }
}

fn split_half(s: &str) -> (String, String) {
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    let mut chars = s.chars();
    let mut has_chars = true;
    while has_chars {
        has_chars = false;
        if let Some(lchar) = chars.next() {
            left.push_back(lchar);
            has_chars = true;
        }
        if let Some(rchar) = chars.next_back() {
            right.push_front(rchar);
            has_chars = true;
        }
    }
    loop {
        if let Some(c) = right.pop_front() {
            left.push_back(c);
            if c.is_whitespace() {
                break;
            }
        }
    }
    (left.into_iter().collect(), right.into_iter().collect())
}
