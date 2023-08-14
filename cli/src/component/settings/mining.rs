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

use crate::{
    component::{
        elements::block_with_title,
        widgets::{LabeledInput, Separator},
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static MINING_SETTINGS: Focus = focus_id!();
static MONERO_ADDRESS: Focus = focus_id!();
static SHA_THREADS: Focus = focus_id!();
static MONERO_URL: Focus = focus_id!();

pub struct MiningSettings {
    expert_sep: Separator,
    monero_address: LabeledInput,
    sha_threads: LabeledInput,
    monero_url: LabeledInput,
}

impl MiningSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            monero_address: LabeledInput::new("Monero mining address", MONERO_ADDRESS),
            sha_threads: LabeledInput::new("SHA3 threads", SHA_THREADS),
            monero_url: LabeledInput::new("Monero node URL", MONERO_URL),
        }
    }
}

impl Input for MiningSettings {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == MINING_SETTINGS {
            state.focus_on(MONERO_ADDRESS);
        } else if state.focus_on == MONERO_ADDRESS {
            let released = self.monero_address.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Down if released => {
                    state.focus_on(SHA_THREADS);
                },
                _ => {
                    self.monero_address.on_event(event, state);
                },
            }
        } else if state.focus_on == SHA_THREADS {
            let released = self.sha_threads.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Up if released => {
                    state.focus_on(MONERO_ADDRESS);
                },
                Pass::Down if released => {
                    state.focus_on(MONERO_URL);
                },
                _ => {
                    self.sha_threads.on_event(event, state);
                },
            }
        } else if state.focus_on == MONERO_URL {
            let released = self.monero_url.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                Pass::Up if released => {
                    state.focus_on(SHA_THREADS);
                },
                _ => {
                    self.monero_url.on_event(event, state);
                },
            }
        } else {
            //
        }
    }
}

impl<B: Backend> Component<B> for MiningSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Mining Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.monero_address.draw(f, chunks[1], state);
        self.sha_threads.draw(f, chunks[2], state);
        self.monero_url.draw(f, chunks[3], state);
    }
}
