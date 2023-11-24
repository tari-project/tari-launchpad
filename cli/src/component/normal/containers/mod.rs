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

use std::{borrow::Cow, cell::RefCell};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Row, Table, TableState},
};
use tari_sdm::ids::{ManagedTask, TaskId};
use tari_sdm_launchpad::resources::images;

use crate::{
    component::{elements::block_with_title, AppState, Component, ComponentEvent, Frame, Input, Pass},
    state::focus,
};

pub struct ContainersScene {
    table_state: RefCell<TableState>,
    containers: Vec<TaskId>,
}

impl ContainersScene {
    pub fn new() -> Self {
        Self {
            table_state: RefCell::new(TableState::default()),
            containers: containers(),
        }
    }
}

impl Input for ContainersScene {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == focus::CONTAINERS_TABLE {
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

impl<B: Backend> Component<B> for ContainersScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Containers"), state.focus_on == focus::CONTAINERS_TABLE);
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(rect);
        let mut rows = Vec::new();
        for task_id in &self.containers {
            if let Some(task_state) = state.state.containers.get(task_id) {
                let col_1 = Cow::Borrowed(task_id.as_ref());
                let mut col_2 = Cow::Borrowed("-");
                let mut col_3 = Cow::Borrowed("-");
                let mut col_4 = Cow::Borrowed("Inactive");
                if task_state.status.is_started() {
                    if let Some(stat_data) = task_state.stats.last() {
                        let cpu_usage = task_state.stats.last_cpu().unwrap_or_default();
                        let usage = format!("{:.2} %", cpu_usage);
                        let mem = stat_data.mem_usage.get_appropriate_unit(false).to_string();
                        col_2 = Cow::Owned(usage);
                        col_3 = Cow::Owned(mem);
                        col_4 = Cow::Owned(task_state.status.to_string());
                    }
                }
                let items = vec![col_1, col_2, col_3, col_4];
                let row = Row::new(items).height(2);
                rows.push(row);
            }
        }
        let header_cells = ["Container", "CPU", "Memory", "Status"];
        let header = Row::new(header_cells)
            .style(Style::default().fg(Color::Yellow))
            .height(1)
            .bottom_margin(1);
        let table = Table::new(rows)
            .block(block)
            .header(header)
            .widths(&[
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(60),
            ])
            .column_spacing(2);
        f.render_stateful_widget(table, rects[0], &mut *self.table_state.borrow_mut());
    }
}

fn containers() -> Vec<TaskId> {
    vec![
        images::Tor::id(),
        images::TariBaseNode::id(),
        images::TariWallet::id(),
        images::TariSha3Miner::id(),
        images::MmProxy::id(),
        images::XMRig::id(),
        images::Grafana::id(),
        images::Loki::id(),
        images::Promtail::id(),
    ]
}
