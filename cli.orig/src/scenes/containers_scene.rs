use std::io::Stdout;

use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};
use tari_launchpad_protocol::{container::TaskState, session::LaunchpadSession};
use tari_sdm::ids::{ManagedTask, TaskId};
use tari_sdm_launchpad::resources::images;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::dashboard::DashboardState;

pub struct ContainersScene<'a, 'b> {
    pub f: &'a mut Frame<'b, CrosstermBackend<Stdout>>,
    pub dashboard_state: &'a DashboardState,
}

impl<'a, 'b> ContainersScene<'a, 'b> {
    pub fn render(&mut self, size: Rect) {
        let mut rows = Vec::new();
        let mut logs = Vec::new();
        let mut selected_container = None;
        if let Some(app_state) = self.dashboard_state.state.as_ref() {
            for container in Container::iter() {
                let id = container.id();
                let selected = container == self.dashboard_state.selected_container;
                let prefix = if selected { "> " } else { "" };
                let name = format!("{}{}", prefix, container);

                if let Some(state) = app_state.containers.get(&id) {
                    let status = format!("{:?}", state.status);
                    let is_active = get_flag(&app_state.config.session, container);
                    let value = (if is_active { "+" } else { "-" }).to_string();
                    let row = Row::new(vec![name, status, value]);
                    rows.push(row);
                    if selected {
                        for line in state.tail.iter().rev() {
                            let item = ListItem::new(line.to_string());
                            logs.push(item);
                        }
                        selected_container = Some(state);
                    }
                } else {
                    let row = Row::new(vec![name, "...".to_string(), "-".to_string()]);
                    rows.push(row);
                }
            }
        }

        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(size);

        let top_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(vchunks[0]);

        self.render_containers(top_row[0], rows);
        self.render_stats(top_row[1], selected_container);
        self.render_logs(vchunks[1], logs);
    }

    fn render_stats(&mut self, size: Rect, state: Option<&TaskState>) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(size);

        let block = Block::default().title("Stats").borders(Borders::ALL);
        if let Some(state) = state {
            let mut rows = Vec::new();
            if let Some(stat_data) = state.stats.last() {
                rows.push(Row::new(vec!["Timestamp".to_string(), stat_data.timestamp.to_string()]));
                let cpu_usage = state.stats.last_cpu().unwrap_or_default();
                rows.push(Row::new(vec!["CPU usage".to_string(), format!("{:.2} %", cpu_usage)]));
                rows.push(Row::new(vec![
                    "Mem limit".to_string(),
                    stat_data.mem_limit.get_appropriate_unit(false).to_string(),
                ]));
                rows.push(Row::new(vec![
                    "Mem usage".to_string(),
                    stat_data.mem_usage.get_appropriate_unit(false).to_string(),
                ]));
                rows.push(Row::new(vec![
                    "Mem usage, %".to_string(),
                    format!("{:.2} %", stat_data.get_mem_pct()),
                ]));
            }
            let table = Table::new(rows)
                .block(block)
                .header(Row::new(vec!["Metric", "Value"]))
                .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)]);
            self.f.render_widget(table, vertical[0]);
        } else {
            self.f.render_widget(block, vertical[0]);
        }
        let block = Block::default().title("Fails").borders(Borders::ALL);
        let mut text = String::new();
        if let Some(err) = state.as_ref().and_then(|state| state.fails.last()) {
            text.push_str(err);
        }
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);
        self.f.render_widget(paragraph, vertical[1]);
    }

    fn render_logs(&mut self, size: Rect, logs: Vec<ListItem<'_>>) {
        let block = Block::default().title("Logs").borders(Borders::ALL);
        let list = List::new(logs).block(block);
        self.f.render_widget(list, size);
    }

    fn render_containers(&mut self, size: Rect, rows: Vec<Row<'_>>) {
        let block = Block::default().title("Containers").borders(Borders::ALL);
        let table = Table::new(rows)
            .block(block)
            .header(Row::new(vec!["Container", "State", "Active"]))
            .widths(&[
                Constraint::Percentage(30),
                Constraint::Percentage(50),
                Constraint::Percentage(20),
            ]);
        self.f.render_widget(table, size);
    }
}

fn get_flag(session: &LaunchpadSession, id: Container) -> bool {
    use Container::*;
    match id {
        Tor => session.is_tor_active(),
        BaseNode => session.is_base_node_active(),
        Wallet => session.is_wallet_active(),
        Miner => session.is_miner_active(),
        MmProxy => session.is_mmproxy_active(),
        Monerod => session.is_monerod_active(),
        XMRig => session.is_xmrig_active(),
        Grafana => session.is_grafana_active(),
        Loki => session.is_loki_active(),
        Promtail => session.is_promtail_active(),
    }
}

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display, PartialEq, Eq)]
pub enum Container {
    Tor,
    BaseNode,
    Wallet,
    Miner,
    MmProxy,
    Monerod,
    XMRig,
    Grafana,
    Loki,
    Promtail,
}

impl Container {
    fn id(self) -> TaskId {
        match self {
            Self::Tor => images::Tor::id(),
            Self::BaseNode => images::TariBaseNode::id(),
            Self::Wallet => images::TariWallet::id(),
            Self::Miner => images::TariSha3Miner::id(),
            Self::MmProxy => images::MmProxy::id(),
            Self::Monerod => images::Monerod::id(),
            Self::XMRig => images::XMRig::id(),
            Self::Grafana => images::Grafana::id(),
            Self::Loki => images::Loki::id(),
            Self::Promtail => images::Promtail::id(),
        }
    }

    fn first() -> Self {
        Self::Tor
    }

    fn last() -> Self {
        Self::Promtail
    }

    pub fn next(&mut self) {
        let repr = *self as usize;
        *self = Self::from_repr(repr + 1).unwrap_or_else(Self::last);
    }

    pub fn prev(&mut self) {
        let repr = *self as usize;
        if repr > 0 {
            *self = Self::from_repr(repr - 1).unwrap_or_else(Self::first);
        } else {
            *self = Self::first();
        }
    }
}
