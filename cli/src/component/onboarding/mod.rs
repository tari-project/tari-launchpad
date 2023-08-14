// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

mod message;

use std::time::{Duration, Instant};

use message::MessageWidget;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Gauge, Paragraph},
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input, Pass},
    state::{focus, onboarding::OnboardingAction, AppState},
};

pub struct OnboardingScene {
    wink: Option<Instant>,
}

impl OnboardingScene {
    pub fn new() -> Self {
        Self {
            wink: Some(Instant::now()),
        }
    }
}

impl Input for OnboardingScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if let Some(wink) = self.wink {
            if wink.elapsed() >= Duration::from_secs(5) {
                self.wink.take();
                state.redraw();
            }
        } else {
            self.wink = Some(Instant::now());
            state.redraw();
        }

        if state.bus.state().is_done() {
            state.focus_on(focus::ROOT);
        }

        match event.pass() {
            Pass::Enter => {
                state.bus.send(OnboardingAction::Next);
            },
            Pass::Leave => {
                state.focus_on(focus::ROOT);
            },
            _ => {},
        }
    }
}

impl OnboardingScene {
    fn get_progress(&self, state: &AppState) -> u16 {
        u16::from(state.bus.state().total_progress.pct)
    }
}

impl<B: Backend> Component<B> for OnboardingScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        let constraints = [
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(h_chunks[1]);

        let constraints = [Constraint::Min(0), Constraint::Length(1)];
        let view_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(v_chunks[1]);

        let msg = state.bus.state().message.clone();
        let message = MessageWidget::new(msg);
        message.draw(f, view_chunks[0], state);

        let constraints = [Constraint::Min(0), Constraint::Length(5), Constraint::Length(5)];
        let line_chinks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(view_chunks[1]);

        let gauge = Gauge::default()
            .label("")
            .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Reset))
            .percent(self.get_progress(state));
        f.render_widget(gauge, line_chinks[0]);
        let style = Style::default().fg(Color::White);
        let bot_state = if self.wink.is_some() { "[o o]" } else { "[- -]" };
        let text = vec![Line::from(Span::styled(bot_state, style))];
        let bot = Paragraph::new(text);
        f.render_widget(bot, line_chinks[2]);
    }
}
