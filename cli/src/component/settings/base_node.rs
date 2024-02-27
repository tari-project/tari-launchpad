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

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::component::widgets::qr_code::QrCodePreview;
use crate::{
    component::{
        elements::block_with_title,
        widgets::{LabeledInput, Separator},
        Component, ComponentEvent, Frame, Input, Pass,
    },
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static BASE_NODE_SETTINGS: Focus = focus_id!();
static ROOT_FOLDER: Focus = focus_id!();

pub struct BaseNodeSettings {
    expert_sep: Separator,
    root_folder: LabeledInput,
    qr_sep: Separator,
    qr_code: QrCodePreview,
}

impl BaseNodeSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            root_folder: LabeledInput::new("Root folder", ROOT_FOLDER),
            qr_sep: Separator::new("QR Code", []),
            qr_code: QrCodePreview::new(),
        }
    }
}

impl Input for BaseNodeSettings {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == BASE_NODE_SETTINGS {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Down | Pass::Enter => {
                    state.focus_on(ROOT_FOLDER);
                },
                _ => {},
            }
        } else if state.focus_on == ROOT_FOLDER {
            let released = self.root_folder.is_released();
            match event.pass() {
                Pass::Up | Pass::Down | Pass::Leave if released => {
                    state.focus_on(BASE_NODE_SETTINGS);
                },
                _ => {
                    self.root_folder.on_event(event, state);
                },
            }
        } else {
            //
        }
        None
    }
}

impl<B: Backend> Component<B> for BaseNodeSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("BaseNode Settings"), state.focus_on == BASE_NODE_SETTINGS);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::default(),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.root_folder.draw(f, chunks[1], state);
        self.qr_sep.draw(f, chunks[2], state);
        self.qr_code.draw(f, chunks[3], state);
    }
}
