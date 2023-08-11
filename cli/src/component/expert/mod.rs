mod containers;
mod logs;

use containers::ContainersScene;
use logs::LogsScene;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use strum::{Display, EnumCount, EnumIter, FromRepr};

use crate::{
    component::{
        tabs::{AppTabs, TabGetter},
        Component,
        ComponentEvent,
        Frame,
        Input,
    },
    state::{focus, AppState, Focus},
};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum ExpertTabs {
    Performance,
    Containers,
    Logs,
}

impl TabGetter for ExpertTabs {
    fn focus_to(&self, _: &AppState) -> Focus {
        focus::ROOT
    }
}

pub struct ExpertScene {
    expert_tabs: AppTabs<ExpertTabs>,
    logs_scene: LogsScene,
    containers_scene: ContainersScene,
}

impl ExpertScene {
    pub fn new() -> Self {
        Self {
            expert_tabs: AppTabs::new(),
            logs_scene: LogsScene::new(),
            containers_scene: ContainersScene::new(),
        }
    }
}

impl Input for ExpertScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.expert_tabs.on_event(event, state);
        match self.expert_tabs.selected() {
            ExpertTabs::Performance => {},
            ExpertTabs::Containers => {
                self.containers_scene.on_event(event, state);
            },
            ExpertTabs::Logs => {
                self.logs_scene.on_event(event, state);
            },
        }
    }
}

impl<B: Backend> Component<B> for ExpertScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.expert_tabs.draw(f, chunks[0], state);
        match self.expert_tabs.selected() {
            ExpertTabs::Performance => {},
            ExpertTabs::Containers => {
                self.containers_scene.draw(f, chunks[1], state);
            },
            ExpertTabs::Logs => {
                self.logs_scene.draw(f, chunks[1], state);
            },
        }
    }
}
