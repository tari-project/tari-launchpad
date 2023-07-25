use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table},
    Frame,
};

use crate::dashboard::DashboardState;

pub struct WalletScene<'a, 'b> {
    pub f: &'a mut Frame<'b, CrosstermBackend<Stdout>>,
    pub dashboard_state: &'a DashboardState,
}

impl<'a, 'b> WalletScene<'a, 'b> {
    pub fn render(&mut self, size: Rect) {
        let wallet_active = self
            .dashboard_state
            .wallet_state()
            .map(|w| w.active)
            .unwrap_or_default();

        // TODO: Allow controls to transfer funds

        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(size);
        let block = Block::default().title("Stats").borders(Borders::ALL);
        let list = List::new(vec![]).block(block);
        self.f.render_widget(list, vchunks[0]);

        let mut rows = Vec::new();
        let mut logs = Vec::new();
        if let Some(app_state) = self.dashboard_state.state.as_ref() {
            if let Some(balance) = app_state.wallet.balance.as_ref() {
                let available = Row::new(vec!["Available".to_string(), balance.available.to_string()]);
                rows.push(available);
                let incoming = Row::new(vec!["Incoming".to_string(), balance.pending_incoming.to_string()]);
                rows.push(incoming);
                let outgoing = Row::new(vec!["Outgoing".to_string(), balance.pending_outgoing.to_string()]);
                rows.push(outgoing);
            }
            for wt in &app_state.wallet.transactions {
                let data = format!("{:?}", wt);
                let item = ListItem::new(data);
                logs.push(item);
            }
        }

        let block = Block::default().title("Wallet").borders(Borders::ALL);
        if wallet_active {
            if rows.is_empty() {
                let pg = Paragraph::new("Balance is not loaded.").block(block);
                self.f.render_widget(pg, vchunks[0]);
            } else {
                let table = Table::new(rows)
                    .block(block)
                    .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);
                self.f.render_widget(table, vchunks[0]);
            }
        } else {
            let pg = Paragraph::new("Wallet is not connected yet.").block(block);
            self.f.render_widget(pg, vchunks[0]);
        }

        let block = Block::default().title("Transactions").borders(Borders::ALL);
        let list = List::new(logs).block(block);
        self.f.render_widget(list, vchunks[1]);
    }
}
