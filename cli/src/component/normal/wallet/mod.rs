mod container;
mod password;

use container::WalletContainerWidget;
use password::PasswordWidget;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct WalletScene {
    password: PasswordWidget,
    container: WalletContainerWidget,
}

impl WalletScene {
    pub fn new() -> Self {
        Self {
            password: PasswordWidget::new(),
            container: WalletContainerWidget::new(),
        }
    }
}

impl Input for WalletScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        // TODO: Check the wallet is locked/unlocked
        self.container.on_event(event, state);
        self.password.on_event(event, state);
    }
}

impl<B: Backend> Component<B> for WalletScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Percentage(50), Constraint::Min(0)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        // self.hint.draw(f, v_chunks[0], state);

        let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(v_chunks[1]);
        self.container.draw(f, h_chunks[0], state);
        // self.password.draw(f, h_chunks[0], state);
    }
}