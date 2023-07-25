use std::{io::Stdout, time::Duration};

use anyhow::Error;
use async_trait::async_trait;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tact::{Actor, ActorContext, Do, Interval, Recipient, Task};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    component::{Component, ComponentEvent, Input, MainView},
    events::{EventHandle, TermEvent},
    state::{bus::Bus, AppState, StateAction},
};

type Term = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Error)]
pub enum DashboardError {
    #[error("Terminal is not connected")]
    NoTerminal,
    #[error("Events thread is not started")]
    NoEvents,
    #[error("The state is not set")]
    NoState,
}

pub enum DashboardEvent {
    Terminated,
}

pub struct Dashboard {
    terminal: Option<Term>,
    event_handle: Option<EventHandle>,
    main_view: MainView,
    // TODO: Get the state from a bus
    state: Option<AppState>,
    interval: Option<Interval>,
    supervisor: Recipient<DashboardEvent>,
    bus: Bus,
    changes: Option<Task>,
}

impl Dashboard {
    pub fn new(bus: Bus, supervisor: Recipient<DashboardEvent>) -> Self {
        Self {
            terminal: None,
            event_handle: None,
            main_view: MainView::new(),
            state: None,
            interval: None,
            supervisor,
            bus,
            changes: None,
        }
    }
}

#[async_trait]
impl Actor for Dashboard {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let notifier = ctx.notifier(Redraw);
        let task = self.bus.changes(notifier);
        self.changes = Some(task);
        let bus = self.bus.clone();
        self.state = Some(AppState::new(bus));
        let notifier = ctx.notifier(Tick);
        let interval = Interval::spawn(Duration::from_millis(250), notifier);
        self.interval = Some(interval);
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        self.terminal = Some(terminal);
        let addr = ctx.address().clone();
        let handle = EventHandle::new(addr);
        self.event_handle = Some(handle);
        ctx.do_next(Redraw)?;
        Ok(())
    }

    async fn finalize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        disable_raw_mode()?;
        let mut terminal = self.terminal.take().ok_or_else(|| DashboardError::NoTerminal)?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        terminal.show_cursor()?;
        self.supervisor.send(DashboardEvent::Terminated)?;
        Ok(())
    }
}

#[async_trait]
impl Do<TermEvent> for Dashboard {
    type Error = Error;

    async fn handle(&mut self, event: TermEvent, ctx: &mut ActorContext<Self>) -> Result<(), Self::Error> {
        match event {
            TermEvent::Event(event) => {
                if let Event::Key(key) = event {
                    if let KeyCode::Char('q') = key.code {
                        self.event_handle
                            .as_mut()
                            .ok_or_else(|| DashboardError::NoEvents)?
                            .interrupt();
                    }
                    let state = self.state.as_mut().ok_or_else(|| DashboardError::NoState)?;
                    self.main_view.on_event(key.into(), state);
                    let changed = state.process_events();
                    if changed {
                        ctx.do_next(Redraw)?;
                    }
                }
                ctx.do_next(Redraw)?;
            },
            TermEvent::End => {
                ctx.shutdown();
            },
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Tick;

#[async_trait]
impl Do<Tick> for Dashboard {
    type Error = Error;

    async fn handle(&mut self, _event: Tick, ctx: &mut ActorContext<Self>) -> Result<(), Self::Error> {
        let state = self.state.as_mut().ok_or_else(|| DashboardError::NoState)?;
        self.main_view.on_event(ComponentEvent::Tick, state);
        let changed = state.process_events();
        if changed {
            ctx.do_next(Redraw)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Redraw;

#[async_trait]
impl Do<Redraw> for Dashboard {
    type Error = Error;

    async fn handle(&mut self, _event: Redraw, _ctx: &mut ActorContext<Self>) -> Result<(), Self::Error> {
        let state = self.state.as_ref().ok_or_else(|| DashboardError::NoState)?;
        let terminal = self.terminal.as_mut().ok_or_else(|| DashboardError::NoTerminal)?;
        terminal.draw(|f| {
            self.main_view.draw(f, f.size(), state);
        })?;
        Ok(())
    }
}

#[async_trait]
impl Do<StateAction> for Dashboard {
    type Error = Error;

    async fn handle(&mut self, event: StateAction, ctx: &mut ActorContext<Self>) -> Result<(), Self::Error> {
        match event {
            StateAction::Redraw => {
                // let state = self.state.as_mut().ok_or_else(|| DashboardError::NoState)?;
                // self.main_view.on_event(KeyEvent::None.into(), state);
                // let changed = state.process_events();
                // if changed {
                // ctx.do_next(Redraw)?;
                // }
                ctx.do_next(Redraw)?;
            },
        }
        Ok(())
    }
}
