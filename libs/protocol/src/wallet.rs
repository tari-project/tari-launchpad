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

use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use log::*;
use serde::{Deserialize, Serialize};
use tari_common_types::tari_address::TariAddress;
use thiserror::Error;

use crate::tari_format::TariFormat;

const HISTORY_LIMIT: usize = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyIdentity {
    pub tari_address: String,
    pub emoji_id: String,
}

#[derive(Debug, Clone, Error)]
pub struct InvalidPublicKey(pub String);

impl Display for InvalidPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Received data is not a valid public key: ")?;
        f.write_str(&self.0)?;
        Ok(())
    }
}

impl From<InvalidPublicKey> for String {
    fn from(v: InvalidPublicKey) -> String {
        v.0
    }
}

impl TryFrom<&[u8]> for MyIdentity {
    type Error = InvalidPublicKey;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let tari_address =
            TariAddress::from_bytes(value).map_err(|e| InvalidPublicKey(format!("Not a valid public key. {e}")))?;
        let emoji_id = tari_address.to_emoji_string();
        let tari_address = tari_address.to_string();
        Ok(Self { tari_address, emoji_id })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletState {
    pub wallet_id: Option<MyIdentity>,
    /// If wallet is active transactions could be sent.
    pub active: bool,
    pub balance: Option<WalletBalance>,
    pub transactions: VecDeque<WalletTransaction>,
    // The set of transactions that have been mined in the current session, but not confirmed yet.
    pub mined_transactions: HashMap<String, WalletTransaction>,
    // The sum total of XTR confirmed mined in this session.
    pub session_confirmed_mined: u64,
    // The sum total of XTR pending mined in this session.
    pub session_pending: u64,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            wallet_id: None,
            active: false,
            balance: None,
            transactions: VecDeque::with_capacity(HISTORY_LIMIT),
            mined_transactions: HashMap::new(),
            session_confirmed_mined: 0,
            session_pending: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletBalance {
    pub available: TariFormat,
    pub pending_incoming: TariFormat,
    pub pending_outgoing: TariFormat,
    pub timelocked: TariFormat,
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
                self.check_mined_transactions(&trans);
                self.transactions.push_front(trans);
            },
        }
    }

    // The mining flow goes like this:
    // 1. The wallet sends a coinbase transaction to the miner. This generates a tx with an id. If you're doing both
    // MM and SHA3 mining, you'll get one tx for each, at the same block height.
    // 2. The miner starts mining. If a block is found elsewhere, a `cancelled` tx is sent. We should remove the
    // tx from the list of mined transactions.
    // 3. If the miner finds a block, it sends a `mined` tx. We can now provisionally add this to our mining
    // balance.
    // 4. After 3 confirmations, we get a `confirmed` tx. We can now add this to our confirmed balance.
    //
    // If event is "received", it means we're mining against that block. We should add this, because when the wallet
    // starts up, there will be many 'cancellation' messages that are stale, potentially screwing up our session
    // balance.
    // If the event is "cancelled", then we should remove the tx from the list of mined transactions, and deduct
    // the pending balance, if the tx was "Mined Unconfirmed".
    // If event is "mined", then we have N confirmations, and we can lock the balance in.
    // If the event is "confirmed", then we're one-step close to being "mined"
    fn check_mined_transactions(&mut self, tx: &WalletTransaction) {
        if !(tx.is_coinbase && tx.direction == "Inbound") {
            return;
        }
        let tx_id = tx.tx_id.clone();
        match tx.event.as_str() {
            "received" => {
                debug!("🤑 Sent new coinbase for mining coinbase. {}", tx.message);
                if let Some(old_tx) = self.mined_transactions.get(&tx_id) {
                    debug!("Replaced another transaction with the same id. {old_tx:?}");
                }
            },
            "confirmed" => {
                if !self.mined_transactions.contains_key(&tx_id) {
                    debug!("Ignoring confirmed tx as a stale transaction. {tx:?}");
                    return;
                }
                let old_tx = self.mined_transactions.insert(tx_id, tx.clone()).unwrap();
                if old_tx.event == "received" {
                    info!("✔️ Got first confirmation on a new block. {old_tx:?}");
                    self.session_pending += tx.amount;
                }
            },
            "mined" => {
                debug!("Found mined block. {}", tx.message);
                if !self.mined_transactions.contains_key(&tx_id) {
                    debug!("Ignoring mined tx as a stale transaction. {tx:?}");
                    return;
                }
                let old_tx = self.mined_transactions.insert(tx_id, tx.clone()).unwrap();
                if old_tx.event == "confirmed" {
                    debug!("Moving pending amount into confirmed. Replacing {old_tx:?}");
                    self.session_pending = self.session_pending.saturating_sub(old_tx.amount);
                }
                if old_tx.event == "received" {
                    info!("🤔 Got a 'mined' status before any 'confirmed'. {old_tx:?}");
                }
                self.session_confirmed_mined += tx.amount;
                info!("💰💰💰 New block mined. {}", tx.message);
            },
            "cancelled" => {
                info!("😞 Cancelled. {}", tx.message);
                if let Some(old_tx) = self.mined_transactions.remove(&tx_id) {
                    if ["mined", "confirmed"].contains(&old_tx.event.as_str()) {
                        debug!("Removed transaction from mined list, and adjusting pending balance {old_tx:?}");
                        self.session_pending = self.session_pending.saturating_sub(old_tx.amount);
                    }
                }
            },
            _ => {},
        }
        trace!("[Wallet gRPC] Transaction info. {tx:?}");
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
