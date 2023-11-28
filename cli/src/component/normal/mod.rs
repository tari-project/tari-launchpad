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

mod base_node;
pub mod containers;
mod mining;
mod wallet;

use mining::MiningScene;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use wallet::WalletScene;

use crate::{
    component::{
        normal::{base_node::BaseNodeWidget, containers::ContainersScene},
        Component,
        ComponentEvent,
        Frame,
        Input,
    },
    state::AppState,
};

pub struct NormalScene {
    mining_scene: MiningScene,
    base_node_widget: BaseNodeWidget,
    wallet_scene: WalletScene,
    containers_scene: ContainersScene,
}

impl NormalScene {
    pub fn new() -> Self {
        Self {
            mining_scene: MiningScene::new(),
            base_node_widget: BaseNodeWidget::new(),
            wallet_scene: WalletScene::new(),
            containers_scene: ContainersScene::new(),
        }
    }
}

impl Input for NormalScene {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        self.base_node_widget.on_event(event, state);
        self.mining_scene.on_event(event, state);
        self.wallet_scene.on_event(event, state);
        None
    }
}

impl<B: Backend> Component<B> for NormalScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let window_constraints = [Constraint::Percentage(60), Constraint::Percentage(40)];
        let panel_constraints = [
            Constraint::Length(17), // miners
            Constraint::Length(10), // base node
            Constraint::Length(16), // wallet
            Constraint::Min(0),
            Constraint::Length(3),
        ];

        let windows = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(window_constraints)
            .split(rect);

        let panels = Layout::default()
            .direction(Direction::Vertical)
            .constraints(panel_constraints)
            .split(windows[0]);

        self.containers_scene.draw(f, windows[1], state);
        self.mining_scene.draw(f, panels[0], state);
        self.base_node_widget.draw(f, panels[1], state);
        self.wallet_scene.draw(f, panels[2], state);
    }
}
