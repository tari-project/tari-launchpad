// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use ratatui::{
    backend::Backend,
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph},
};
use tari_launchpad_protocol::tari_format::TariFormat;

use crate::{component::Component, state::AppState};

pub struct SessionStats {
    pub total_confirmed: TariFormat,
    pub total_pending: TariFormat,
    pub blocks_mined: u64,
}

pub struct SessionStatWidget;

impl<B: Backend> Component<B> for SessionStatWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let stats = calculate_stats(state);
        let block = Block::default()
            .title("Session Stats")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(2));
        let confirmed = Line::styled(
            format!("Total mined (confirmed): {:12}", stats.total_confirmed),
            Style::default().bold(),
        );
        let pending = Line::from(format!("Total mined (pending): {:12}", stats.total_pending));
        let blocks_mined = Line::from(format!("Blocks found: {}", stats.blocks_mined));
        let p = Paragraph::new(Text::from(vec![confirmed, pending, blocks_mined])).block(block);
        f.render_widget(p, rect);
    }
}

fn calculate_stats(state: &AppState) -> SessionStats {
    let session = &state.state.wallet;
    let total_confirmed = TariFormat::from(session.session_confirmed_mined);
    let total_pending = TariFormat::from(session.session_pending);
    let blocks_mined = session
        .mined_transactions
        .values()
        .filter(|tx| tx.event == "mined")
        .count() as u64;
    SessionStats {
        total_confirmed,
        total_pending,
        blocks_mined,
    }
}
