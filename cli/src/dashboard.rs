use std::{io::Stdout, time::Duration};

use anyhow::Error;
use async_trait::async_trait;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tact::{Actor, ActorContext, Do, Interval, Recipient, Task};
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction, Reaction};
use tari_sdm_launchpad::bus::{BusTx, LaunchpadBus};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    component::{Component, ComponentEvent, Input, MainView, TerminationView},
    events::{EventHandle, TermEvent},
    state::{bus::Bus, focus, AppState},
};

type Term = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Error)]
pub enum DashboardError {
    #[error("Terminal is not connected")]
    Terminal,
    #[error("Events thread is not started")]
    Events,
    #[error("The state is not set")]
    State,
}

pub enum DashboardEvent {
    Terminated,
}

pub struct Dashboard {
    terminal: Option<Term>,
    event_handle: Option<EventHandle>,
    main_view: MainView,
    termination_view: TerminationView,
    // TODO: Get the state from a bus
    state: Option<AppState>,
    interval: Option<Interval>,
    supervisor: Recipient<DashboardEvent>,
    bus: Bus,
    bus_tx: Option<BusTx>,
    bus_deltas: Option<Task>,
    changes: Option<Task>,
}

impl Dashboard {
    pub fn new(bus: Bus, supervisor: Recipient<DashboardEvent>) -> Self {
        Self {
            terminal: None,
            event_handle: None,
            main_view: MainView::new(),
            termination_view: TerminationView::new(),
            state: None,
            interval: None,
            supervisor,
            bus,
            bus_tx: None,
            bus_deltas: None,
            changes: None,
        }
    }
}

#[async_trait]
impl Actor for Dashboard {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        self.init_bus(ctx)?;

        let notifier = ctx.notifier(Redraw);
        let task = self.bus.changes(notifier);
        self.changes = Some(task);

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

        self.connect_to_bus()?;

        ctx.do_next(Redraw)?;
        Ok(())
    }

    async fn finalize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        disable_raw_mode()?;
        let mut terminal = self.terminal.take().ok_or_else(|| DashboardError::Terminal)?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        terminal.show_cursor()?;
        self.supervisor.send(DashboardEvent::Terminated)?;
        Ok(())
    }
}

impl Dashboard {
    fn init_bus(&mut self, ctx: &ActorContext<Self>) -> Result<(), Error> {
        let mut lp_bus = LaunchpadBus::start()?;
        self.bus_tx = Some(lp_bus.incoming);
        let recipient = ctx.recipient();
        let task = Task::spawn(async move {
            while let Some(action) = lp_bus.outgoing.recv().await {
                if let Err(err) = recipient.send(action) {
                    log::error!("Can't send an action from the bus to the dashboard: {err}");
                    break;
                }
            }
        });
        self.bus_deltas = Some(task);

        Ok(())
    }

    fn connect_to_bus(&mut self) -> Result<(), Error> {
        log::info!("Connecting to the bus");
        let tx = self
            .bus_tx
            .as_mut()
            .ok_or_else(|| Error::msg("Bus is not initialized"))?;
        let action = Action::Action(LaunchpadAction::Connect);
        tx.send(action)?;
        Ok(())
    }

    fn stop_the_app(&mut self) -> Result<(), Error> {
        self.event_handle.as_mut().ok_or(DashboardError::Events)?.interrupt();
        Ok(())
    }
}

#[async_trait]
impl Do<Reaction> for Dashboard {
    type Error = Error;

    async fn handle(&mut self, event: Reaction, ctx: &mut ActorContext<Self>) -> Result<(), Self::Error> {
        log::trace!("Processing the event: {event:?}");
        match event {
            Reaction::State(state) => {
                let bus = self.bus.clone();
                let bus_tx = self
                    .bus_tx
                    .clone()
                    .ok_or_else(|| Error::msg("No bus sender available"))?;
                self.state = Some(AppState::new(bus, bus_tx, state));
            },
            Reaction::Delta(delta) => {
                if let Some(state) = self.state.as_mut() {
                    state.state.apply(delta);
                }
            },
        }
        ctx.do_next(Redraw)?;
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
                    let state = self.state.as_mut().ok_or_else(|| DashboardError::State)?;
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
        let state = self.state.as_mut().ok_or_else(|| DashboardError::State)?;
        self.main_view.on_event(ComponentEvent::Tick, state);
        let changed = state.process_events();
        if changed {
            ctx.do_next(Redraw)?;
        }
        if state.is_terminated() {
            self.stop_the_app()?;
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
        let state = self.state.as_ref().ok_or_else(|| DashboardError::State)?;
        let terminal = self.terminal.as_mut().ok_or_else(|| DashboardError::Terminal)?;
        terminal.draw(|f| {
            let rect = f.size();
            if state.focus_on == focus::TERMINATION {
                self.termination_view.draw(f, rect, state);
            } else {
                self.main_view.draw(f, rect, state);
            }
        })?;
        Ok(())
    }
}
