mod dashboard;

use anyhow::Error;
use crossterm::event::{Event, KeyCode};
use dashboard::Dashboard;
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction};
use tari_sdm_launchpad::LaunchpadBus;
use tokio::{
    select,
    time::{sleep, Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // env_logger::try_init()?;
    let mut bus = LaunchpadBus::start()?;
    let action = Action::Action(LaunchpadAction::Connect);
    bus.incoming.send(action)?;

    let mut dashboard = Dashboard::init()?;
    dashboard.render()?;

    let mut last_render = Instant::now();
    loop {
        select! {
            _ = sleep(Duration::from_millis(800)) => {
            }
            event = bus.outgoing.recv() => {
                if let Some(event) = event {
                    dashboard.process_delta(event);
                }
            }
            event = dashboard.next_event() => {
                if let Some(Event::Key(key)) = event {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('s') => {
                            let action = Action::Start;
                            bus.incoming.send(action)?;
                        },
                        key => {
                            dashboard.process_key(key);
                            dashboard.render()?;
                        }
                    }
                }
            }
        }
        if last_render.elapsed() >= Duration::from_millis(300) {
            dashboard.render()?;
            last_render = Instant::now();
        }
    }
    dashboard.uninit()?;
    Ok(())
}

/// Create a cryptographically secure password on length `len`
pub fn create_password(len: usize) -> String {
    use rand::distributions::{Alphanumeric, Distribution};
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_iter(&mut rng).take(len).map(char::from).collect()
}
