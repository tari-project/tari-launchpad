pub mod bus;
pub mod focus;
pub mod onboarding;

use std::collections::VecDeque;

pub use focus::Focus;
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction, LaunchpadState};
use tari_sdm_launchpad::bus::BusTx;

use crate::state::bus::Bus;

pub enum AppEvent {
    SetFocus(Focus),
    UpdateState,
    Redraw,
}

pub struct AppState {
    pub focus_on: Focus,
    pub events_queue: VecDeque<AppEvent>,
    pub bus: Bus,
    pub bus_tx: BusTx,
    pub state: LaunchpadState,
}

impl AppState {
    pub fn new(bus: Bus, bus_tx: BusTx, state: LaunchpadState) -> Self {
        Self {
            focus_on: focus::ROOT,
            events_queue: VecDeque::new(),
            bus,
            bus_tx,
            state,
        }
    }

    pub fn focus_on(&mut self, value: Focus) {
        let event = AppEvent::SetFocus(value);
        self.events_queue.push_front(event);
    }

    pub fn redraw(&mut self) {
        let event = AppEvent::Redraw;
        self.events_queue.push_front(event);
    }

    pub fn update_state(&mut self) {
        let event = AppEvent::UpdateState;
        self.events_queue.push_front(event);
    }

    pub fn process_events(&mut self) -> bool {
        if self.events_queue.is_empty() {
            false
        } else {
            for event in self.events_queue.drain(..) {
                match event {
                    AppEvent::SetFocus(value) => {
                        self.focus_on = value;
                    },
                    AppEvent::UpdateState => {
                        let new_session = self.state.config.session.clone();
                        let event = LaunchpadAction::ChangeSession(new_session);
                        let action = Action::Action(event);
                        if let Err(err) = self.bus_tx.send(action) {
                            log::error!("Can't update the state: {err}");
                        }
                    },
                    AppEvent::Redraw => {},
                }
            }
            true
        }
    }
}
