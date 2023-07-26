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

pub struct DockerSettings {
    expert_sep: Separator,
    docker_tag: LabeledInput,
    docker_registry: LabeledInput,
    statuses_sep: Separator,
}

impl DockerSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            docker_tag: LabeledInput::new("Docker Tag"),
            docker_registry: LabeledInput::new("Docker Registry"),
            statuses_sep: Separator::new("Image Statuses", []),
        }
    }
}

impl Input for DockerSettings {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for DockerSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Docker Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [
            // Expert
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            // Image Statuses
            Constraint::Length(1),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.docker_tag.draw(f, chunks[1], state);
        self.docker_registry.draw(f, chunks[2], state);

        self.statuses_sep.draw(f, chunks[3], state);
    }
}
