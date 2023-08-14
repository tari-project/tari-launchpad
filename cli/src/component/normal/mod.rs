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
mod chrono_button;
mod hint;
mod mining;
mod wallet;

use base_node::BaseNodeScene;
use mining::MiningScene;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};
use strum::{Display, EnumCount, EnumIter, FromRepr};
use wallet::WalletScene;

use crate::{
    component::{
        tabs::{AppTabs, TabGetter},
        Component,
        ComponentEvent,
        Frame,
        Input,
    },
    state::{focus, AppState, Focus},
};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum NormalTabs {
    Mining,
    #[strum(serialize = "Base Node")]
    BaseNode,
    Wallet,
}

impl TabGetter for NormalTabs {
    fn get_badge(&self, state: &AppState) -> Option<(&str, Color)> {
        if let Self::Mining = self {
            if state.state.config.session.is_mmproxy_active() {
                return Some(("(running)", Color::Green));
            }
        }
        None
    }

    fn focus_to(&self, _: &AppState) -> Focus {
        match self {
            Self::Mining => focus::TARI_MINING,
            Self::BaseNode => focus::BASE_NODE,
            Self::Wallet => focus::WALLET_CONTAINER,
        }
    }
}

pub struct NormalScene {
    normal_tabs: AppTabs<NormalTabs>,
    mining_scene: MiningScene,
    base_node_scene: BaseNodeScene,
    wallet_scene: WalletScene,
}

impl NormalScene {
    pub fn new() -> Self {
        Self {
            normal_tabs: AppTabs::new(),
            mining_scene: MiningScene::new(),
            base_node_scene: BaseNodeScene::new(),
            wallet_scene: WalletScene::new(),
        }
    }
}

impl Input for NormalScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.normal_tabs.on_event(event, state);
        match self.normal_tabs.selected() {
            NormalTabs::Mining => {
                self.mining_scene.on_event(event, state);
            },
            NormalTabs::BaseNode => {
                self.base_node_scene.on_event(event, state);
            },
            NormalTabs::Wallet => {
                self.wallet_scene.on_event(event, state);
            },
        }
    }
}

impl<B: Backend> Component<B> for NormalScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.normal_tabs.draw(f, chunks[0], state);
        match self.normal_tabs.selected() {
            NormalTabs::Mining => {
                self.mining_scene.draw(f, chunks[1], state);
            },
            NormalTabs::BaseNode => {
                self.base_node_scene.draw(f, chunks[1], state);
            },
            NormalTabs::Wallet => {
                self.wallet_scene.draw(f, chunks[1], state);
            },
        }
    }
}
