use tui::{backend::Backend, layout::Rect};

use crate::{
    component::{elements::block_with_title, Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct Wallet {}

impl Wallet {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for Wallet {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for Wallet {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(Some("Wallet"), false);
        f.render_widget(block, rect);
    }
}
