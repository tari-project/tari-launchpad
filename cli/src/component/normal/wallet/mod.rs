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

mod balance;
mod container;
mod password;
mod send_funds;

use balance::BalanceWidget;
pub use balance::BALANCE;
use container::WalletContainerWidget;
pub use container::WALLET_CONTAINER;
use password::PasswordWidget;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use send_funds::SendFundsWidget;
pub use send_funds::SEND_FUNDS;

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct WalletScene {
    password: PasswordWidget,
    container: WalletContainerWidget,
    balance: BalanceWidget,
    send_funds: SendFundsWidget,
}

impl WalletScene {
    pub fn new() -> Self {
        Self {
            password: PasswordWidget::new(),
            container: WalletContainerWidget::new(),
            balance: BalanceWidget::new(),
            send_funds: SendFundsWidget::new(),
        }
    }
}

impl Input for WalletScene {
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        // TODO: Check the wallet is locked/unlocked
        self.container.on_event(event, state);
        self.password.on_event(event, state);
        self.balance.on_event(event, state);
        self.send_funds.on_event(event, state);
        None
    }
}

impl<B: Backend> Component<B> for WalletScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Percentage(40), Constraint::Min(0)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        // self.hint.draw(f, v_chunks[0], state);

        let constraints = [Constraint::Percentage(60), Constraint::Percentage(40)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(v_chunks[1]);
        self.container.draw(f, h_chunks[0], state);
        self.balance.draw(f, h_chunks[1], state);
        self.send_funds.draw(f, v_chunks[2], state);
    }
}
