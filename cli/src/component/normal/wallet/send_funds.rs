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

use anyhow::Error;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};
use tari_launchpad_protocol::wallet::{TransferFunds, WalletAction};

use crate::{
    component::{
        elements::block_with_title,
        normal::wallet::BALANCE,
        widgets::{ChronoButton, ChronoGetter, LabeledInput, ModalDialog},
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    focus_id,
    state::{focus, focus::Focus, AppState},
};

pub static SEND_FUNDS: Focus = focus_id!();
static ADDRESS: Focus = focus_id!();
static AMOUNT: Focus = focus_id!();
static FEE: Focus = focus_id!();
static MESSAGE: Focus = focus_id!();
static BUTTON: Focus = focus_id!();
static DIALOG: Focus = focus_id!();

struct SendFundsGetter;

impl ChronoGetter for SendFundsGetter {
    fn get_label(&self, _state: &AppState) -> &str {
        "Send"
    }
}

pub struct SendFundsWidget {
    // TODO: Validation is possible here
    address: LabeledInput<String>,
    amount: LabeledInput<u64>,
    fee: LabeledInput<u64>,
    message: LabeledInput<String>,
    button: ChronoButton<SendFundsGetter>,
    dialog: ModalDialog,
}

impl SendFundsWidget {
    pub fn new() -> Self {
        Self {
            address: LabeledInput::new("To", ADDRESS),
            amount: LabeledInput::new_with_filter("Amount", AMOUNT, char::is_numeric),
            fee: LabeledInput::new_with_filter("Fee", FEE, char::is_numeric),
            message: LabeledInput::new("Message", MESSAGE),
            button: ChronoButton::new(SendFundsGetter, BUTTON),
            dialog: ModalDialog::new(DIALOG),
        }
    }
}

impl Input for SendFundsWidget {
    type Output = ();

    // TODO: Split to separate methods
    #[allow(clippy::too_many_lines)]
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
        if state.focus_on == SEND_FUNDS {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(focus::WALLET);
                },
                Pass::Enter | Pass::Down | Pass::Right => {
                    state.focus_on(ADDRESS);
                },
                Pass::Left => {
                    state.focus_on(BALANCE);
                },
                _ => {},
            }
        } else if state.focus_on == ADDRESS {
            let released = self.address.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave | Pass::Left if released => {
                    state.focus_on(SEND_FUNDS);
                },
                Pass::Down | Pass::Right if released => {
                    state.focus_on(AMOUNT);
                },
                _ => {
                    self.address.on_event(event, state);
                },
            }
        } else if state.focus_on == AMOUNT {
            let released = self.amount.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(SEND_FUNDS);
                },
                Pass::Up | Pass::Left if released => {
                    state.focus_on(ADDRESS);
                },
                Pass::Down | Pass::Right if released => {
                    state.focus_on(FEE);
                },
                _ => {
                    self.amount.on_event(event, state);
                },
            }
        } else if state.focus_on == FEE {
            let released = self.fee.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(SEND_FUNDS);
                },
                Pass::Up | Pass::Left if released => {
                    state.focus_on(AMOUNT);
                },
                Pass::Down | Pass::Right if released => {
                    state.focus_on(MESSAGE);
                },
                _ => {
                    self.fee.on_event(event, state);
                },
            }
        } else if state.focus_on == MESSAGE {
            let released = self.message.is_released();
            match event.pass() {
                Pass::Leave if released => {
                    state.focus_on(SEND_FUNDS);
                },
                Pass::Up | Pass::Left if released => {
                    state.focus_on(FEE);
                },
                Pass::Down | Pass::Right if released => {
                    state.focus_on(BUTTON);
                },
                _ => {
                    self.message.on_event(event, state);
                },
            }
        } else if state.focus_on == BUTTON {
            match event.pass() {
                Pass::Leave => {
                    state.focus_on(SEND_FUNDS);
                },
                Pass::Up | Pass::Left => {
                    state.focus_on(MESSAGE);
                },
                Pass::Down | Pass::Right => {
                    state.focus_on(SEND_FUNDS);
                },
                Pass::Enter => match self.get_transfer() {
                    Ok(action) => {
                        let wallet_action = WalletAction::TransferFunds(action);
                        state.send_action(wallet_action);
                        self.dialog.show_success(state, &"Transaction Sent");
                    },
                    Err(err) => {
                        self.dialog.show_error(state, &err);
                    },
                },
                _ => {
                    self.button.on_event(event, state);
                },
            }
        } else if state.focus_on == DIALOG {
            self.dialog.on_event(event, state);
        } else {
            //
        }
        None
    }
}

impl SendFundsWidget {
    fn get_transfer(&self) -> Result<TransferFunds, Error> {
        Ok(TransferFunds {
            address: self.address.value()?.clone(),
            amount: *self.amount.value()?,
            fee: *self.fee.value()?,
            message: self.message.value()?.clone(),
        })
    }
}

impl<B: Backend> Component<B> for SendFundsWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Send Funds"), state.focus_on == SEND_FUNDS);
        let inner_rect = block.inner(rect);
        let constraints = [
            Constraint::Length(3),
            Constraint::Length(3),
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
        self.address.draw(f, chunks[0], state);
        self.amount.draw(f, chunks[1], state);
        self.fee.draw(f, chunks[2], state);
        self.message.draw(f, chunks[3], state);
        self.button.draw(f, chunks[4], state);
        f.render_widget(block, rect);
        self.dialog.draw(f, f.size(), state);
    }
}
