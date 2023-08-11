use std::time::Duration;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{
        elements::{block_with_title, logo},
        normal::chrono_button::{ChronoButton, ChronoGetter},
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    state::{focus, AppState},
};

const LOGO: &str = r#"
╦ ╦┌─┐┬  ┬  ┌─┐┌┬┐
║║║├─┤│  │  ├┤  │
╚╩╝┴ ┴┴─┘┴─┘└─┘ ┴
"#;

struct WalletContainerGetter;

impl ChronoGetter for WalletContainerGetter {
    fn get_duration(&self, _state: &AppState) -> Option<Duration> {
        None
    }

    fn get_label(&self, state: &AppState) -> &str {
        if state.state.config.session.is_wallet_active() {
            "Pause"
        } else {
            "Start wallet"
        }
    }
}

pub struct WalletContainerWidget {
    button: ChronoButton<WalletContainerGetter>,
}

impl WalletContainerWidget {
    pub fn new() -> Self {
        Self {
            button: ChronoButton::new(WalletContainerGetter),
        }
    }
}

impl Input for WalletContainerWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == focus::WALLET_CONTAINER {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Enter | Pass::Space => {
                    let session = &mut state.state.config.session;
                    session.wallet_active = !session.wallet_active;
                    state.update_state();
                },
                _ => {},
            }
        }
    }
}

impl<B: Backend> Component<B> for WalletContainerWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Wallet"), state.focus_on == focus::WALLET_CONTAINER);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            // Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        // self.status_badge.draw(f, v_chunks[0], state);

        let logo = logo(LOGO);
        f.render_widget(logo, v_chunks[1]);

        // self.tari_amount.draw(f, v_chunks[2], state);

        self.button.draw(f, v_chunks[4], state);
    }
}
