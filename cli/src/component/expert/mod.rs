use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

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
}

impl ExpertScene {
    pub fn new() -> Self {
        Self {
            expert_tabs: AppTabs::new(),
        }
    }
}

impl Input for ExpertScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.expert_tabs.on_event(event, state);
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
    }
}
