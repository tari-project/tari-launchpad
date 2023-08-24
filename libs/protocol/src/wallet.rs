// Copyright 2022. The Tari Project
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

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

const HISTORY_LIMIT: usize = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyIdentity {
    pub tari_address: String,
    pub emoji_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletState {
    pub wallet_id: Option<MyIdentity>,
    /// If wallet is active transactions could be sent.
    pub active: bool,
    pub balance: Option<WalletBalance>,
    pub transactions: VecDeque<WalletTransaction>,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            wallet_id: None,
            active: false,
            balance: None,
            transactions: VecDeque::with_capacity(HISTORY_LIMIT),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletBalance {
    pub available: u64,
    pub pending_incoming: u64,
    pub pending_outgoing: u64,
    pub timelocked: u64,
}

impl WalletState {
    pub fn apply(&mut self, delta: WalletDelta) {
        match delta {
            WalletDelta::SetAddress(wallet_id) => {
                self.wallet_id = Some(wallet_id);
            },
            WalletDelta::SetActive(flag) => {
                self.active = flag;
                if !self.active {
                    self.balance.take();
                }
            },
            WalletDelta::UpdateBalance(balance) => {
                self.balance = Some(balance);
            },
            WalletDelta::LogTransaction(trans) => {
                if self.transactions.len() >= HISTORY_LIMIT {
                    self.transactions.pop_back();
                }
                self.transactions.push_front(trans);
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletDelta {
    SetAddress(MyIdentity),
    SetActive(bool),
    UpdateBalance(WalletBalance),
    LogTransaction(WalletTransaction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransaction {
    pub event: String,
    pub tx_id: String,
    pub status: String,
    pub direction: String,
    pub amount: u64,
    pub message: String,
    pub is_coinbase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletAction {
    TransferFunds(TransferFunds),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFunds {
    pub address: String,
    pub amount: u64,
    pub fee: u64,
    pub message: String,
}
