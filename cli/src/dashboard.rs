use std::io::Stdout;

use anyhow::Error;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tari_launchpad_protocol::launchpad::{LaunchpadState, Reaction};
use tokio::sync::mpsc;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Row, Table},
    Terminal,
};

pub struct Dashboard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    events: mpsc::Receiver<Event>,
    state: Option<LaunchpadState>,
    selected_container: usize,
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
        Ok(Self {
            terminal,
            events: rx,
            state: None,
            selected_container: 0,
        })
    }

    pub fn process_delta(&mut self, reaction: Reaction) {
        match reaction {
            Reaction::State(state) => {
                self.state = Some(state);
            },
            Reaction::Delta(delta) => {
                if let Some(state) = self.state.as_mut() {
                    state.apply(delta);
                }
            },
        }
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        self.events.recv().await
    }

    pub fn process_key(&mut self, key: KeyCode) {
        let total = self
            .state
            .as_ref()
            .map(|state| state.containers.len())
            .unwrap_or_default();
        match key {
            KeyCode::Up => {
                if self.selected_container > 0 {
                    self.selected_container -= 1;
                }
            },
            KeyCode::Down => {
                if self.selected_container < total {
                    self.selected_container += 1;
                }
            },
            _ => {},
        }
    }

    pub fn uninit(mut self) -> Result<(), Error> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        let mut rows = Vec::new();
        let mut logs = Vec::new();
        if let Some(state) = self.state.as_ref() {
            for (idx, (id, state)) in state.containers.iter().enumerate() {
                let selected = idx == self.selected_container;
                let prefix = if selected { "> " } else { "" };
                let name = format!("{}{}", prefix, id);
                let status = format!("{:?}", state.status);
                let row = Row::new(vec![name, status]);
                rows.push(row);
                if selected {
                    for line in &state.tail {
                        let item = ListItem::new(line.to_string());
                        logs.push(item);
                    }
                }
            }
        }
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                //.margin(4)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(f.size());

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);

            let block = Block::default().title("Logs").borders(Borders::ALL);
            let list = List::new(logs).block(block);
            f.render_widget(list, chunks[1]);

            let block = Block::default().title("Containers").borders(Borders::ALL);
            let table = Table::new(rows)
                .block(block)
                .header(Row::new(vec!["Container", "State"]))
                .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)]);

            f.render_widget(table, top_chunks[0]);
        })?;
        Ok(())
    }
}
