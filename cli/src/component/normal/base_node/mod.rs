mod base_node;

use base_node::BaseNodeWidget;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{
        normal::hint::{HintGetter, HintLine},
        Component,
        ComponentEvent,
        Frame,
        Input,
    },
    state::AppState,
};

struct BaseNodeHint;

impl HintGetter for BaseNodeHint {
    fn get_hint(&self, _state: &AppState) -> String {
        "Base Node is already running!".into()
    }
}

pub struct BaseNodeScene {
    hint: HintLine<BaseNodeHint>,
    base_node: BaseNodeWidget,
}

impl BaseNodeScene {
    pub fn new() -> Self {
        Self {
            hint: HintLine::new(BaseNodeHint),
            base_node: BaseNodeWidget::new(),
        }
    }
}

impl Input for BaseNodeScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.base_node.on_event(event, state);
    }
}

impl<B: Backend> Component<B> for BaseNodeScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Percentage(50), Constraint::Min(0)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.hint.draw(f, v_chunks[0], state);

        let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(v_chunks[1]);
        self.base_node.draw(f, h_chunks[0], state);
        // self.merged_mining.draw(f, h_chunks[1], state);
    }
}
