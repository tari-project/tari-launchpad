use tui::{backend::Backend, layout::Rect};

use crate::{
    component::{elements::block_with_title, Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct Containers {}

impl Containers {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for Containers {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for Containers {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(Some("Containers"), false);
        f.render_widget(block, rect);
    }
}
