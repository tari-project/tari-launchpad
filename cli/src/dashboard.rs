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
    wallet::WalletState,
};
use tokio::sync::mpsc;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
    Terminal,
};

use crate::scenes::{
    containers_scene::{Container, ContainersScene},
    wallet_scene::WalletScene,
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

pub struct DashboardState {
    pub state: Option<LaunchpadState>,
    pub selected_container: Container,
    pub selected_tab: Tab,
    pub show_help: bool,
}

impl DashboardState {
    pub fn state(&self) -> Option<&LaunchpadState> {
        self.state.as_ref()
    }

    pub fn wallet_state(&self) -> Option<&WalletState> {
        self.state().map(|state| &state.wallet)
    }
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
                    .any(|state| state.status.is_started())
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
                if let Some(state) = self.dashboard_state.state() {
                    let mut session = state.config.session.clone();
                    session.all_active = !session.all_active;
                    return Some(session);
                }
            },
            KeyCode::Char('q') => {
                if let Some(state) = self.dashboard_state.state() {
                    let mut session = state.config.session.clone();
                    session.stop_all();
                    return Some(session);
                }
            },
            KeyCode::Char(' ') => {
                if let Some(state) = self.dashboard_state.state() {
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
                    let mut scene = ContainersScene {
                        f: self.f,
                        dashboard_state: self.dashboard_state,
                    };
                    scene.render(rect);
                },
                Tab::Wallet => {
                    let mut scene = WalletScene {
                        f: self.f,
                        dashboard_state: self.dashboard_state,
                    };
                    scene.render(rect);
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
