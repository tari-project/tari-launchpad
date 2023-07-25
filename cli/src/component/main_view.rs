use tui::{
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
        scene,
        settings::SettingsScene,
        Component,
        ComponentEvent,
        Input,
    },
    state::{focus, AppState},
};

pub struct MainView {
    header: Header,
    normal_scene: NormalScene,
    expert_scene: ExpertScene,
    settings_scene: SettingsScene,
    containers_scene: scene::Containers,
    wallet_scene: scene::Wallet,
    onboarding_scene: OnboardingScene,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            normal_scene: NormalScene::new(),
            expert_scene: ExpertScene::new(),
            settings_scene: SettingsScene::new(),
            containers_scene: scene::Containers::new(),
            wallet_scene: scene::Wallet::new(),
            onboarding_scene: OnboardingScene::new(),
        }
    }
}

impl Input for MainView {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
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
