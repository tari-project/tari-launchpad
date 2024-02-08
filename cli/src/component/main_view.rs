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

use crossterm::terminal::disable_raw_mode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::component::expert::logs::LogsScene;
use crate::{
    component::{
        expert::ExpertScene,
        header::{mode::Mode, Header},
        normal::NormalScene,
        settings::SettingsScene,
        widgets::docker_detect::is_docker_running,
        Component, ComponentEvent, Input, Pass,
    },
    state::{focus, AppState},
};

pub struct MainView {
    header: Header,
    normal_scene: NormalScene,
    logs_scene: LogsScene,
    expert_scene: ExpertScene,
    settings_scene: SettingsScene,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            normal_scene: NormalScene::new(),
            logs_scene: LogsScene::new(),
            expert_scene: ExpertScene::new(),
            settings_scene: SettingsScene::new(),
        }
    }
}

impl Input for MainView {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if event.pass() == Pass::Quit {
            let session = &mut state.state.config.session;
            session.stop_all();
            state.terminate();
            state.focus_on(focus::TERMINATION);
            state.update_state();

            // Spawn a new thread to exit the process after 30s if it has not already exited
            if is_docker_running() {
                std::thread::spawn(|| {
                    std::thread::sleep(std::time::Duration::from_secs(60));
                    log::warn!("The process did not stop cleanly. Terminating it.");
                    let _unused = disable_raw_mode();
                    std::process::exit(0);
                });
            } else {
                let _unused = disable_raw_mode();
                std::process::exit(0);
            }
        } else if matches!(event, ComponentEvent::StateChanged) {
            self.normal_scene.on_event(event, state);
            self.settings_scene.on_event(event, state);
        } else {
            self.header.on_event(event, state);
            match self.header.mode_selector.selected() {
                Mode::Normal => {
                    self.normal_scene.on_event(event, state);
                },
                Mode::Logs => {
                    self.logs_scene.on_event(event, state);
                },
                Mode::Expert => {
                    self.expert_scene.on_event(event, state);
                },
                Mode::Settings => {
                    self.settings_scene.on_event(event, state);
                },
            }
        }
        None
    }
}

impl<B: Backend> Component<B> for MainView {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.header.draw(f, chunks[0], state);
        match self.header.mode_selector.selected() {
            Mode::Normal => {
                self.normal_scene.draw(f, chunks[1], state);
            },
            Mode::Logs => {
                self.logs_scene.draw(f, chunks[1], state);
            },
            Mode::Expert => {
                self.expert_scene.draw(f, chunks[1], state);
            },
            Mode::Settings => {
                self.settings_scene.draw(f, chunks[1], state);
            },
        }
    }
}
