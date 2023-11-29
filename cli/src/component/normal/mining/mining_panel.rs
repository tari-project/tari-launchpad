// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Padding},
};

use crate::{
    component::{
        normal::mining::{
            helpers::{MergeMiningStatus, ShaMiningStatus},
            session_stats::SessionStatWidget,
            status_badge::StatusBadge,
        },
        Component,
        ComponentEvent,
        ComponentEvent::KeyEvent,
        Input,
        Pass,
    },
    state::AppState,
};

pub struct MiningPanel {
    mm_status: StatusBadge<MergeMiningStatus>,
    sha3_status: StatusBadge<ShaMiningStatus>,
    session_stats: SessionStatWidget,
}

impl MiningPanel {
    pub fn new() -> Self {
        Self {
            mm_status: StatusBadge::new(MergeMiningStatus),
            sha3_status: StatusBadge::new(ShaMiningStatus),
            session_stats: SessionStatWidget,
        }
    }

    fn toggle_merge_mining(state: &mut AppState) {
        let session = &mut state.state.config.session;
        session.merge_layer_active = !session.merge_layer_active;
        state.update_state();
    }

    fn toggle_sha3_mining(state: &mut AppState) {
        let session = &mut state.state.config.session;
        session.sha3x_layer_active = !session.sha3x_layer_active;
        state.update_state();
    }
}

impl Input for MiningPanel {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if let KeyEvent(key) = event {
            if key.code == KeyCode::Char('m') || key.code == KeyCode::Char('M') {
                Self::toggle_merge_mining(state);
                return Some(());
            }
            if key.code == KeyCode::Char('t') || key.code == KeyCode::Char('T') {
                Self::toggle_sha3_mining(state);
                return Some(());
            }
        }
        match event.pass() {
            Pass::Tick => {
                if state.state.config.session.is_sha3x_active() {
                    state.redraw();
                }
            },
            _ => {},
        }
        None
    }
}

impl<B: Backend> Component<B> for MiningPanel {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let mining = state.state.config.session.is_sha3x_active() || state.state.config.session.is_xmrig_active();
        let block = title_block(mining);
        let inner_rect = block.inner(rect);

        let v_constraints = [
            Constraint::Length(1), // Merged mining status
            Constraint::Length(1), // SHA3x mining status
            Constraint::Max(1),    // stretch
            Constraint::Length(5), // Session stats
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(v_constraints)
            .split(inner_rect);

        f.render_widget(block, rect);
        self.mm_status.draw(f, v_chunks[0], state);
        self.sha3_status.draw(f, v_chunks[1], state);
        self.session_stats.draw(f, v_chunks[3], state);
    }
}

fn title_block(highlight: bool) -> Block<'static> {
    let mut border_style = Style::default().add_modifier(Modifier::BOLD);
    if highlight {
        border_style = border_style.fg(Color::Green);
    }
    Block::default()
        .border_type(BorderType::Thick)
        .border_style(border_style)
        .borders(Borders::ALL)
        .padding(Padding::uniform(1))
        .title("Mining")
}
