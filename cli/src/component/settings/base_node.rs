use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{
        elements::block_with_title,
        widgets::{LabeledInput, Separator},
        Component,
        ComponentEvent,
        Frame,
        Input,
    },
    state::AppState,
};

pub struct BaseNodeSettings {
    expert_sep: Separator,
    root_folder: LabeledInput,
}

impl BaseNodeSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            root_folder: LabeledInput::new("Root folder"),
        }
    }
}

impl Input for BaseNodeSettings {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for BaseNodeSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("BaseNode Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [Constraint::Length(1), Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.root_folder.draw(f, chunks[1], state);
    }
}
