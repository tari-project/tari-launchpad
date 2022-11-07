use std::io::Stdout;

use anyhow::Error;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};
use tari_launchpad_protocol::{
    launchpad::{LaunchpadState, Reaction},
    session::LaunchpadSession,
};
use tari_sdm::ids::{ManagedTask, TaskId};
use tari_sdm_launchpad::resources::images;
use tokio::sync::mpsc;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, Tabs},
    Frame,
    Terminal,
};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum Tab {
    Containers,
    Wallet,
}

impl Tab {
    fn first() -> Self {
        Self::Containers
    }

    fn last() -> Self {
        Self::Wallet
    }

    fn next(&mut self) {
        let repr = *self as usize;
        *self = Self::from_repr(repr + 1).unwrap_or_else(Self::last);
    }

    fn prev(&mut self) {
        let repr = *self as usize;
        if repr > 0 {
            *self = Self::from_repr(repr - 1).unwrap_or_else(Self::first);
        } else {
            *self = Self::first();
        }
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

    fn next(&mut self) {
        let repr = *self as usize;
        *self = Self::from_repr(repr + 1).unwrap_or_else(Self::last);
    }

    fn prev(&mut self) {
        let repr = *self as usize;
        if repr > 0 {
            *self = Self::from_repr(repr - 1).unwrap_or_else(Self::first);
        } else {
            *self = Self::first();
        }
    }
}

pub struct DashboardState {
    state: Option<LaunchpadState>,
    selected_container: Container,
    selected_tab: Tab,
    show_help: bool,
}

pub struct Dashboard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    events: mpsc::Receiver<Event>,
    dashboard_state: DashboardState,
    terminating: bool,
}

impl Dashboard {
    pub fn init() -> Result<Self, Error> {
        let (tx, rx) = mpsc::channel(12);
        std::thread::spawn(move || -> Result<(), Error> {
            loop {
                let event = event::read()?;
                tx.blocking_send(event)?;
            }
        });
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let dashboard_state = DashboardState {
            state: None,
            selected_container: Container::BaseNode,
            selected_tab: Tab::Containers,
            show_help: false,
        };
        Ok(Self {
            terminal,
            events: rx,
            dashboard_state,
            terminating: false,
        })
    }

    pub fn state(&self) -> Option<&LaunchpadState> {
        self.dashboard_state.state.as_ref()
    }

    pub fn is_alive(&self) -> bool {
        let has_active_task = self
            .dashboard_state
            .state
            .as_ref()
            .map(|state| {
                state
                    .containers
                    .values()
                    .filter(|state| !state.permanent)
                    .any(|state| state.status.is_active())
            })
            .unwrap_or_default();

        !self.terminating || has_active_task
    }

    pub fn terminate(&mut self) {
        self.terminating = true;
    }

    pub fn process_delta(&mut self, reaction: Reaction) {
        match reaction {
            Reaction::State(state) => {
                self.dashboard_state.state = Some(state);
            },
            Reaction::Delta(delta) => {
                if let Some(state) = self.dashboard_state.state.as_mut() {
                    state.apply(delta);
                }
            },
        }
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        self.events.recv().await
    }

    pub fn process_key(&mut self, key: KeyCode) -> Option<LaunchpadSession> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.dashboard_state.selected_container.prev();
            },
            KeyCode::Down | KeyCode::Char('j') => {
                self.dashboard_state.selected_container.next();
            },
            KeyCode::Left | KeyCode::Char('h') => {
                self.dashboard_state.selected_tab.prev();
            },
            KeyCode::Right | KeyCode::Char('l') => {
                self.dashboard_state.selected_tab.next();
            },
            KeyCode::Esc | KeyCode::Tab => {
                self.dashboard_state.show_help = !self.dashboard_state.show_help;
            },
            KeyCode::Char('s') => {
                if let Some(state) = self.state() {
                    let mut session = state.config.session.clone();
                    session.all_active = !session.all_active;
                    return Some(session);
                }
            },
            KeyCode::Char('q') => {
                if let Some(state) = self.state() {
                    let mut session = state.config.session.clone();
                    session.all_active = false;
                    return Some(session);
                }
            },
            KeyCode::Char(' ') => {
                if let Some(state) = self.state() {
                    let session = &state.config.session;
                    let container = self.dashboard_state.selected_container;
                    return Some(toggle_flag(session, container));
                }
            },
            _ => {},
        }
        None
    }

    pub fn uninit(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        self.terminal.draw(|f| {
            let mut render = Render {
                f,
                dashboard_state: &self.dashboard_state,
            };
            render.render();
        })?;
        Ok(())
    }
}

struct Render<'a, 'b> {
    f: &'a mut Frame<'b, CrosstermBackend<Stdout>>,
    dashboard_state: &'a DashboardState,
}

impl<'a, 'b> Render<'a, 'b> {
    fn render(&mut self) {
        let rect = self.render_tabs();
        if self.dashboard_state.show_help {
            self.render_help(rect);
        } else {
            match self.dashboard_state.selected_tab {
                Tab::Containers => {
                    self.render_containers(rect);
                },
                Tab::Wallet => {
                    self.render_wallet(rect);
                },
            }
        }
    }

    fn render_help(&mut self, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(rect);

        let text = vec![Spans::from("S - Start/Stop containers")];

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .title(Span::styled("Help", Style::default().add_modifier(Modifier::BOLD)));
        let paragraph = Paragraph::new(text.clone())
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .block(block)
            .alignment(Alignment::Left);
        self.f.render_widget(paragraph, chunks[0]);
    }

    fn render_tabs(&mut self) -> Rect {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(self.f.size());

        let titles = Tab::iter()
            .map(|s| Spans::from(vec![Span::raw(s.to_string())]))
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.dashboard_state.selected_tab as usize)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Black));
        self.f.render_widget(tabs, main_chunks[0]);
        main_chunks[1]
    }

    fn render_wallet(&mut self, size: Rect) {
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
        let table = Table::new(rows)
            .block(block)
            .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);
        self.f.render_widget(table, vchunks[0]);

        let block = Block::default().title("Transactions").borders(Borders::ALL);
        let list = List::new(logs).block(block);
        self.f.render_widget(list, vchunks[1]);
    }

    fn render_containers(&mut self, size: Rect) {
        let mut rows = Vec::new();
        let mut logs = Vec::new();
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
                        for line in &state.tail {
                            let item = ListItem::new(line.to_string());
                            logs.push(item);
                        }
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

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(vchunks[0]);

        let block = Block::default().title("Logs").borders(Borders::ALL);
        let list = List::new(logs).block(block);
        self.f.render_widget(list, vchunks[1]);

        let block = Block::default().title("Containers").borders(Borders::ALL);
        let table = Table::new(rows)
            .block(block)
            .header(Row::new(vec!["Container", "State", "Flag"]))
            .widths(&[
                Constraint::Percentage(30),
                Constraint::Percentage(60),
                Constraint::Percentage(10),
            ]);
        self.f.render_widget(table, top_chunks[0]);
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

fn toggle_flag(session: &LaunchpadSession, id: Container) -> LaunchpadSession {
    let mut session = session.clone();
    use Container::*;
    match id {
        Tor => {
            session.tor_active = !session.tor_active;
        },
        BaseNode => {
            session.base_node_active = !session.base_node_active;
        },
        Wallet => {
            session.wallet_active = !session.wallet_active;
        },
        Miner => {
            session.miner_active = !session.miner_active;
        },
        MmProxy => {
            session.mmproxy_active = !session.mmproxy_active;
        },
        Monerod => {
            session.monerod_active = !session.monerod_active;
        },
        XMRig => {
            session.xmrig_active = !session.xmrig_active;
        },
        Grafana => {
            session.grafana_active = !session.grafana_active;
        },
        Loki => {
            session.loki_active = !session.loki_active;
        },
        Promtail => {
            session.promtail_active = !session.promtail_active;
        },
    }
    session
}
