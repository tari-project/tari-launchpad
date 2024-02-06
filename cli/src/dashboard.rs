// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::{io::Stdout, time::Duration};

use anyhow::Error;
use async_trait::async_trait;
use crossterm::{
    event::Event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tact::{Actor, ActorContext, Do, Interval, Recipient, Task};
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction, Reaction};
#[cfg(not(feature = "sim"))]
use tari_sdm_launchpad::bus::{BusTx, LaunchpadBus};
#[cfg(feature = "sim")]
use tari_sim_launchpad::bus::{BusTx, LaunchpadBus};
use thiserror::Error;

use crate::{
    component::{
        display_docker_notice, is_docker_running, wait_for_keypress, Component, ComponentEvent, Input, MainView,
        TerminationView,
    },
    events::{EventHandle, TermEvent},
    state::{focus, AppState},
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
    bus_tx: Option<BusTx>,
    bus_deltas: Option<Task>,
}

impl Dashboard {
    pub fn new(supervisor: Recipient<DashboardEvent>) -> Self {
        Self {
            terminal: None,
            event_handle: None,
            main_view: MainView::new(),
            termination_view: TerminationView::new(),
            state: None,
            interval: None,
            supervisor,
            bus_tx: None,
            bus_deltas: None,
        }
    }
}

#[async_trait]
impl Actor for Dashboard {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        self.init_bus(ctx)?;

        let notifier = ctx.notifier(Tick);
        let interval = Interval::spawn(Duration::from_millis(250), notifier);
        self.interval = Some(interval);
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        self.terminal = Some(terminal);
        let addr = ctx.address().clone();
        let handle = EventHandle::new(addr);
        self.event_handle = Some(handle);

        if !is_docker_running() {
            #[cfg(target_os = "macos")]
            let url = "https://docs.docker.com/desktop/install/mac-install/";
            #[cfg(target_os = "windows")]
            let url = "https://docs.docker.com/desktop/install/windows-install/";
            #[cfg(target_os = "linux")]
            let url = "https://docs.docker.com/engine/install/ubuntu/";

            let msg = format!(
                "\nThe Docker process is not detected.\nIs it installed and running?\n\n\
            Download docker at {url}\n\n\
            Press any key to quit."
            );
            if self
                .terminal
                .as_mut()
                .unwrap()
                .draw(|f| display_docker_notice(f, "Docker Not Running!", &msg))
                .is_err()
            {
                println!("{}", msg);
            }

            wait_for_keypress();
            println!();
            println!();
            std::process::exit(0);
        }

        self.connect_to_bus()?;

        ctx.do_next(Redraw)?;
        Ok(())
    }

    async fn finalize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        disable_raw_mode()?;
        let mut terminal = self.terminal.take().ok_or_else(|| DashboardError::Terminal)?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
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
                let bus_tx = self
                    .bus_tx
                    .clone()
                    .ok_or_else(|| Error::msg("No bus sender available"))?;
                self.state = Some(AppState::new(bus_tx, state));
            },
            Reaction::Delta(delta) => {
                if let Some(state) = self.state.as_mut() {
                    state.state.apply(delta);
                }
            },
        }
        // Reporting about the state has changed - this gets triggered for every event, so isn't very efficient.
        if let Some(state) = self.state.as_mut() {
            let event = ComponentEvent::StateChanged;
            self.main_view.on_event(event, state);
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
                    // Don't process key down events, only key up, otherwise repeat rate is too high.
                    #[cfg(windows)]
                    if key.kind != crossterm::event::KeyEventKind::Release {
                        return Ok(());
                    }

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
