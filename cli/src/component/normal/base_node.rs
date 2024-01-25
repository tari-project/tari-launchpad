// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::Padding,
};

use crate::{
    component::{
        elements::block_with_title,
        widgets::status_line::{StatusLine, StatusReportGetter},
        Component, ComponentEvent,
        ComponentEvent::KeyEvent,
        Frame, Input,
    },
    state::AppState,
};

#[derive(Default)]
struct BaseNodeStatus {}

impl BaseNodeStatus {
    /// Returns the status text of the base node. Only called if the state is active.
    fn get_status(&self, state: &AppState) -> Text {
        let node_status = &state.state.node;
        let status_style = match node_status.sync_status.as_str() {
            "Listening" | "Online" => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            _ => Style::default().add_modifier(Modifier::BOLD),
        };
        let sync_state = Span::styled(format!("{:<15}", node_status.sync_status), status_style);
        let chain_height = Span::raw(format!("Height: {:<5}", node_status.chain_height));
        let peer_count = Span::raw(format!("   Peers: {:3}", node_status.peer_count));
        let top_line = Line::from(vec![sync_state, chain_height, peer_count]);
        let public_key = node_status
            .identity
            .as_ref()
            .cloned()
            .map(|id| (id.public_key))
            .unwrap_or_else(|| ("-".into()));
        let mid_line = Line::from(format!("Public key: {public_key}"));
        Text::from(vec![top_line, mid_line])
    }
}

impl StatusReportGetter for BaseNodeStatus {
    fn get_status(&self, state: &AppState) -> Text {
        if state.state.config.session.is_base_node_active() {
            self.get_status(state)
        } else {
            "Not running".into()
        }
    }
}

pub struct BaseNodeWidget {
    status: StatusLine<BaseNodeStatus>,
}

impl BaseNodeWidget {
    pub fn new() -> Self {
        Self {
            status: StatusLine::new(BaseNodeStatus::default()),
        }
    }

    pub fn toggle_base_node(state: &mut AppState) {
        let session = &mut state.state.config.session;
        session.base_layer_active = !session.base_layer_active;
        state.update_state();
    }
}

impl Input for BaseNodeWidget {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if let KeyEvent(key) = event {
            if key.code == KeyCode::Char('b') && key.modifiers.contains(KeyModifiers::CONTROL) {
                Self::toggle_base_node(state);
                return Some(());
            }
        }
        None
    }
}

impl<B: Backend> Component<B> for BaseNodeWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let node_active = state.state.config.session.is_base_node_active();
        let block = block_with_title(Some("Base Node [Ctrl-B]"), node_active).padding(Padding::new(1, 1, 1, 1));
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        //                              Status line,           Gap,               Button
        let constraints = [Constraint::Min(15), Constraint::Length(2), Constraint::Length(16)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner_rect);

        self.status.draw(f, h_chunks[0], state);
    }
}
