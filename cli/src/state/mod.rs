pub mod bus;
pub mod focus;
pub mod launchpad;
pub mod mining;
pub mod onboarding;

use std::collections::VecDeque;

pub use focus::Focus;
use launchpad::LaunchpadState;

use crate::state::bus::Bus;

#[derive(Debug, Clone)]
pub enum StateAction {
    Redraw,
}

pub enum AppEvent {
    SetFocus(Focus),
    Redraw,
}

pub struct AppState {
    pub focus_on: Focus,
    pub events_queue: VecDeque<AppEvent>,
    pub bus: Bus,
    pub launchpad: LaunchpadState,
}

impl AppState {
    pub fn new(bus: Bus) -> Self {
        Self {
            focus_on: focus::ROOT,
            events_queue: VecDeque::new(),
            bus,
            launchpad: LaunchpadState::new(),
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

    pub fn process_events(&mut self) -> bool {
        if self.events_queue.is_empty() {
            false
        } else {
            for event in self.events_queue.drain(..) {
                match event {
                    AppEvent::SetFocus(value) => {
                        self.focus_on = value;
                    },
                    AppEvent::Redraw => {},
                }
            }
            true
        }
    }
}
