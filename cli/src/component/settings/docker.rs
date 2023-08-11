use ratatui::{
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
        Pass,
    },
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static DOCKER_SETTINGS: Focus = focus_id!();
static DOCKER_TAG: Focus = focus_id!();
static DOCKER_REGISTRY: Focus = focus_id!();

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
            docker_tag: LabeledInput::new("Docker Tag", DOCKER_TAG),
            docker_registry: LabeledInput::new("Docker Registry", DOCKER_REGISTRY),
            statuses_sep: Separator::new("Image Statuses", []),
        }
    }
}

impl Input for DockerSettings {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == DOCKER_SETTINGS {
            state.focus_on(DOCKER_TAG);
        } else if state.focus_on == DOCKER_TAG {
            let released = self.docker_tag.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Down if released => {
                    state.focus_on(DOCKER_REGISTRY);
                },
                _ => {
                    self.docker_tag.on_event(event, state);
                },
            }
        } else if state.focus_on == DOCKER_REGISTRY {
            let released = self.docker_registry.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Up if released => {
                    state.focus_on(DOCKER_TAG);
                },
                _ => {
                    self.docker_registry.on_event(event, state);
                },
            }
        } else {
            //
        }
    }
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
