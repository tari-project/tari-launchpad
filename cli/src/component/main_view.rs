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

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    component::{
        expert::ExpertScene,
        header::{mode::Mode, Header},
        normal::NormalScene,
        onboarding::OnboardingScene,
        settings::SettingsScene,
        Component,
        ComponentEvent,
        Input,
        Pass,
    },
    state::{focus, AppState},
};

pub struct MainView {
    header: Header,
    normal_scene: NormalScene,
    expert_scene: ExpertScene,
    settings_scene: SettingsScene,
    onboarding_scene: OnboardingScene,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            normal_scene: NormalScene::new(),
            expert_scene: ExpertScene::new(),
            settings_scene: SettingsScene::new(),
            onboarding_scene: OnboardingScene::new(),
        }
    }
}

impl Input for MainView {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if event.pass() == Pass::Quit {
            let session = &mut state.state.config.session;
            session.stop_all();
            state.terminate();
            state.focus_on(focus::TERMINATION);
            state.update_state();
            return;
        }
        self.header.on_event(event, state);
        if state.focus_on == focus::ONBOARDING {
            self.onboarding_scene.on_event(event, state);
        } else {
            match self.header.mode_selector.selected() {
                Mode::Normal => {
                    self.normal_scene.on_event(event, state);
                },
                Mode::Expert => {
                    self.expert_scene.on_event(event, state);
                },
                Mode::Settings => {
                    self.settings_scene.on_event(event, state);
                },
            }
        }
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
        if state.focus_on == focus::ONBOARDING {
            self.onboarding_scene.draw(f, chunks[1], state);
        } else {
            match self.header.mode_selector.selected() {
                Mode::Normal => {
                    self.normal_scene.draw(f, chunks[1], state);
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
}
