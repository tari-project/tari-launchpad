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

use std::time::Duration;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Padding,
};

use crate::{
    component::{
        elements::{block_with_title, logo},
        widgets::{ChronoButton, ChronoGetter},
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    focus_id,
    state::{focus, AppState, Focus},
};

const LOGO: &str = r#"
╔╗ ┌─┐┌─┐┌─┐  ╔╗╔┌─┐┌┬┐┌─┐
╠╩╗├─┤└─┐├┤   ║║║│ │ ││├┤
╚═╝┴ ┴└─┘└─┘  ╝╚╝└─┘─┴┘└─┘
"#;

static BUTTON: Focus = focus_id!();

struct BaseNodeGetter;

impl ChronoGetter for BaseNodeGetter {
    fn get_duration(&self, _state: &AppState) -> Option<Duration> {
        None
    }

    fn get_label(&self, state: &AppState) -> &str {
        if state.state.config.session.is_base_node_active() {
            "Pause"
        } else {
            "Start node"
        }
    }
}

pub struct BaseNodeWidget {
    button: ChronoButton<BaseNodeGetter>,
}

impl BaseNodeWidget {
    pub fn new() -> Self {
        Self {
            button: ChronoButton::new(BaseNodeGetter, BUTTON),
        }
    }
}

impl Input for BaseNodeWidget {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == focus::BASE_NODE {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Enter | Pass::Space => {
                    let session = &mut state.state.config.session;
                    session.base_layer_active = !session.base_layer_active;
                    state.update_state();
                },
                _ => {},
            }
        }
        None
    }
}

impl<B: Backend> Component<B> for BaseNodeWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block =
            block_with_title(Some("Base Node"), state.focus_on == focus::BASE_NODE).padding(Padding::new(1, 1, 1, 0));
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(3),
            // Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        // self.status_badge.draw(f, v_chunks[0], state);

        let logo = logo(LOGO);
        f.render_widget(logo, v_chunks[0]);

        // self.tari_amount.draw(f, v_chunks[1], state);

        self.button.draw(f, v_chunks[3], state);
    }
}
