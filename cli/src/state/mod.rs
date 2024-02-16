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

pub mod focus;

use std::collections::VecDeque;

use anyhow::Error;
pub use focus::Focus;
use tari_launchpad_protocol::launchpad::{Action, LaunchpadAction, LaunchpadState};
use tari_sdm_launchpad::bus::BusTx;

pub enum AppEvent {
    SetFocus(Focus),
    SettingsChanged,
    UpdateState,
}

pub struct AppState {
    pub focus_on: Focus,
    pub events_queue: VecDeque<AppEvent>,
    pub bus_tx: BusTx,
    pub state: LaunchpadState,
    pub terminate: bool,
}

impl AppState {
    pub fn new(bus_tx: BusTx, state: LaunchpadState) -> Self {
        Self {
            focus_on: focus::BASE_NODE,
            events_queue: VecDeque::new(),
            bus_tx,
            state,
            terminate: false,
        }
    }

    pub fn is_terminated(&mut self) -> bool {
        let has_active_task = self
            .state
            .containers
            .values()
            .filter(|state| !state.permanent)
            .any(|state| state.status.is_started());
        self.terminate && !has_active_task
    }

    pub fn focus_on(&mut self, value: Focus) {
        let event = AppEvent::SetFocus(value);
        self.events_queue.push_front(event);
    }

    pub fn terminate(&mut self) {
        self.terminate = true;
    }

    pub fn update_state(&mut self) {
        let event = AppEvent::UpdateState;
        self.events_queue.push_front(event);
    }

    pub fn update_settings(&mut self) {
        let event = AppEvent::SettingsChanged;
        self.events_queue.push_front(event);
    }

    pub fn process_events(&mut self) -> bool {
        if self.events_queue.is_empty() {
            false
        } else {
            if let Err(err) = self.process_events_impl() {
                log::error!("Can't update the state: {err}");
            }
            true
        }
    }

    pub fn process_events_impl(&mut self) -> Result<(), Error> {
        for event in self.events_queue.drain(..) {
            match event {
                AppEvent::SetFocus(value) => {
                    self.focus_on = value;
                },
                AppEvent::UpdateState => {
                    let new_session = self.state.config.session.clone();
                    let event = LaunchpadAction::ChangeSession(new_session);
                    let action = Action::Action(event);
                    self.bus_tx.send(action)?;
                },
                AppEvent::SettingsChanged => {
                    let settings = self
                        .state
                        .config
                        .settings
                        .as_ref()
                        .map(|s| s.saved_settings.clone())
                        .ok_or_else(|| {
                            Error::msg(
                                "Can't update settings, because the app state does not have a settings instance \
                                 configured",
                            )
                        })?;
                    let action = Action::Action(LaunchpadAction::SaveSettings(Box::new(settings)));
                    self.bus_tx.send(action)?;
                },
            }
        }
        Ok(())
    }
}
