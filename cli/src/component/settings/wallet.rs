use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{elements::block_with_title, widgets::LabeledInput, Component, ComponentEvent, Frame, Input, Pass},
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static WALLET_SETTINGS: Focus = focus_id!();
static WALLET_ID: Focus = focus_id!();

pub struct WalletSettings {
    wallet_id: LabeledInput,
}

impl WalletSettings {
    pub fn new() -> Self {
        Self {
            wallet_id: LabeledInput::new("Tari Wallet ID (address)", WALLET_ID),
        }
    }
}

impl Input for WalletSettings {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == WALLET_SETTINGS {
            state.focus_on(WALLET_ID);
        } else if state.focus_on == WALLET_ID {
            let released = self.wallet_id.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                _ => {
                    self.wallet_id.on_event(event, state);
                },
            }
        } else {
            //
        }
    }
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
