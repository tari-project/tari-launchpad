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
╔╦╗┌─┐┬─┐┌─┐┌─┐┌┬┐  ╔╦╗┬┌┐┌┬┌┐┌┌─┐
║║║├┤ ├┬┘│ ┬├┤  ││  ║║║│││││││││ ┬
╩ ╩└─┘┴└─└─┘└─┘─┴┘  ╩ ╩┴┘└┘┴┘└┘└─┘
"#;

struct MergedMiningGetter;

impl StatusGetter for MergedMiningGetter {
    fn get_status(&self, state: &AppState) -> (&str, Color) {
        if state.launchpad.merged_mining.is_active() {
            ("(Running)", Color::Green)
        } else {
            ("(Ready to set)", Color::Cyan)
        }
    }
}

impl ChronoGetter for MergedMiningGetter {
    fn get_duration(&self, state: &AppState) -> Option<Duration> {
        state.launchpad.merged_mining.mining_duration()
    }

    fn get_label(&self, state: &AppState) -> &str {
        if state.launchpad.merged_mining.mining_duration().is_some() {
            "Pause"
        } else {
            "Start mining"
        }
    }
}

struct XtrGetter;

impl AmountGetter for XtrGetter {
    fn get_amount(&self, state: &AppState) -> (Decimal, &str) {
        let amount = state.launchpad.merged_mining.tari_amount;
        (amount, "XTR")
    }
}

struct XmrGetter;

impl AmountGetter for XmrGetter {
    fn get_amount(&self, state: &AppState) -> (Decimal, &str) {
        let amount = state.launchpad.merged_mining.monero_amount;
        (amount, "XMR")
    }
}

pub struct MergedMiningWidget {
    status_badge: StatusBadge<MergedMiningGetter>,
    tari_amount: AmountIndicator<XtrGetter>,
    monero_amount: AmountIndicator<XmrGetter>,
    button: ChronoButton<MergedMiningGetter>,
}

impl MergedMiningWidget {
    pub fn new() -> Self {
        Self {
            status_badge: StatusBadge::new(MergedMiningGetter),
            tari_amount: AmountIndicator::new(XtrGetter),
            monero_amount: AmountIndicator::new(XmrGetter),
            button: ChronoButton::new(MergedMiningGetter),
        }
    }
}

impl Input for MergedMiningWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == focus::MERGED_MINING {
            match event.pass() {
                Pass::Left | Pass::Next => {
                    state.focus_on(focus::TARI_MINING);
                },
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Enter | Pass::Space => {
                    state.launchpad.merged_mining.toggle();
                },
                Pass::Tick => {
                    if state.launchpad.merged_mining.is_active() {
                        state.redraw();
                    }
                },
                _ => {},
            }
        }
    }
}

impl<B: Backend> Component<B> for MergedMiningWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Merged Mining"), state.focus_on == focus::MERGED_MINING);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            // Constraint::Percentage(50),
            Constraint::Length(1),
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
        self.monero_amount.draw(f, v_chunks[3], state);

        self.button.draw(f, v_chunks[5], state);
    }
}
