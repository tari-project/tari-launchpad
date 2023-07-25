use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{elements::block_with_title, widgets::LabeledInput, Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct WalletSettings {
    wallet_id: LabeledInput,
}

impl WalletSettings {
    pub fn new() -> Self {
        Self {
            wallet_id: LabeledInput::new("Tari Wallet ID (address)"),
        }
    }
}

impl Input for WalletSettings {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for WalletSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Wallet Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.wallet_id.draw(f, chunks[0], state);
    }
}
