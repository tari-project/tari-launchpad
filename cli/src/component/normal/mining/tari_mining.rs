use std::time::Duration;

use rust_decimal::Decimal;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

use crate::{
    component::{
        elements::{block_with_title, logo},
        normal::{
            chrono_button::{ChronoButton, ChronoGetter},
            mining::{
                amount::{AmountGetter, AmountIndicator},
                status_badge::{StatusBadge, StatusGetter},
            },
        },
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    state::{focus, AppState},
};

const LOGO: &str = r#"
╔╦╗┌─┐┬─┐┬  ╔╦╗┬┌┐┌┬┌┐┌┌─┐
 ║ ├─┤├┬┘│  ║║║│││││││││ ┬
 ╩ ┴ ┴┴└─┴  ╩ ╩┴┘└┘┴┘└┘└─┘
"#;

struct TariMiningGetter;

impl StatusGetter for TariMiningGetter {
    fn get_status(&self, state: &AppState) -> (&str, Color) {
        if state.launchpad.tari_mining.is_active() {
            ("(Running)", Color::Green)
        } else {
            ("(Ready to set)", Color::Cyan)
        }
    }
}

impl ChronoGetter for TariMiningGetter {
    fn get_duration(&self, state: &AppState) -> Option<Duration> {
        state.launchpad.tari_mining.mining_duration()
    }

    fn get_label(&self, state: &AppState) -> &str {
        if state.launchpad.tari_mining.mining_duration().is_some() {
            "Pause"
        } else {
            "Start mining"
        }
    }
}

struct XtrGetter;

impl AmountGetter for XtrGetter {
    fn get_amount(&self, state: &AppState) -> (Decimal, &str) {
        let amount = state.launchpad.tari_mining.tari_amount;
        (amount, "XTR")
    }
}

pub struct TariMiningWidget {
    status_badge: StatusBadge<TariMiningGetter>,
    tari_amount: AmountIndicator<XtrGetter>,
    button: ChronoButton<TariMiningGetter>,
}

impl TariMiningWidget {
    pub fn new() -> Self {
        Self {
            status_badge: StatusBadge::new(TariMiningGetter),
            tari_amount: AmountIndicator::new(XtrGetter),
            button: ChronoButton::new(TariMiningGetter),
        }
    }
}

impl Input for TariMiningWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == focus::TARI_MINING {
            match event.pass() {
                Pass::Right | Pass::Next => {
                    state.focus_on(focus::MERGED_MINING);
                },
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Enter | Pass::Space => {
                    state.launchpad.tari_mining.toggle();
                },
                Pass::Tick => {
                    if state.launchpad.tari_mining.is_active() {
                        state.redraw();
                    }
                },
                _ => {},
            }
        }
    }
}

impl<B: Backend> Component<B> for TariMiningWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Tari Mining"), state.focus_on == focus::TARI_MINING);
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
        self.status_badge.draw(f, v_chunks[0], state);

        let logo = logo(LOGO);
        f.render_widget(logo, v_chunks[1]);

        self.tari_amount.draw(f, v_chunks[2], state);

        self.button.draw(f, v_chunks[4], state);
    }
}
