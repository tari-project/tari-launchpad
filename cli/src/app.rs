use anyhow::Error;
use crossterm::event::{Event, KeyCode};
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction};
#[cfg(not(feature = "sim"))]
use tari_sdm_launchpad::LaunchpadBus;
#[cfg(feature = "sim")]
use tari_sim_launchpad::LaunchpadBus;
use tokio::{
    select,
    time::{sleep, Duration, Instant},
};

use crate::Dashboard;

pub struct App {
    bus: LaunchpadBus,
    dashboard: Dashboard,
    last_render: Instant,
}

impl App {
    pub fn init() -> Result<Self, Error> {
        let bus = LaunchpadBus::start()?;
        let dashboard = Dashboard::init()?;
        Ok(Self {
            bus,
            dashboard,
            last_render: Instant::now(),
        })
    }

    /// Connect to a bus.
    pub fn connect(&mut self) -> Result<(), Error> {
        let action = Action::Action(LaunchpadAction::Connect);
        self.bus.incoming.send(action)?;
        Ok(())
    }

    pub async fn routine(&mut self) -> Result<(), Error> {
        self.connect()?;
        self.dashboard.render()?;
        // TODO: Check that state is inactive
        while self.dashboard.is_alive() {
            self.step().await?;
        }
        self.dashboard.uninit()?;
        Ok(())
    }

    pub async fn step(&mut self) -> Result<(), Error> {
        select! {
            _ = sleep(Duration::from_millis(800)) => {
            }
            event = self.bus.outgoing.recv() => {
                if let Some(event) = event {
                    self.dashboard.process_delta(event);
                }
            }
            event = self.dashboard.next_event() => {
                if let Some(Event::Key(key)) = event {
                    if let KeyCode::Char('q') = key.code {
                        self.dashboard.terminate();
                    }
                    if let Some(new_session) = self.dashboard.process_key(key.code) {
                        let event = LaunchpadAction::ChangeSession(new_session);
                        let action = Action::Action(event);
                        self.bus.incoming.send(action)?;
                    }
                    self.dashboard.render()?;
                }
            }
        }
        if self.last_render.elapsed() >= Duration::from_millis(300) {
            self.dashboard.render()?;
            self.last_render = Instant::now();
        }
        Ok(())
    }
}
