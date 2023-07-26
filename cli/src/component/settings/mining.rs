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

pub struct MiningSettings {
    expert_sep: Separator,
    monero_address: LabeledInput,
    sha_threads: LabeledInput,
    monero_url: LabeledInput,
}

impl MiningSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            monero_address: LabeledInput::new("Monero mining address"),
            sha_threads: LabeledInput::new("SHA3 threads"),
            monero_url: LabeledInput::new("Monero node URL"),
        }
    }
}

impl Input for MiningSettings {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for MiningSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Mining Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.monero_address.draw(f, chunks[1], state);
        self.sha_threads.draw(f, chunks[2], state);
        self.monero_url.draw(f, chunks[3], state);
    }
}
