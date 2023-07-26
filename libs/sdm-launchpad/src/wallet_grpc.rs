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

use anyhow::Error;
use futures::StreamExt;
use tari_launchpad_protocol::{
    launchpad::{LaunchpadDelta, Reaction},
    wallet::{WalletAction, WalletBalance, WalletDelta, WalletTransaction},
};
use tari_wallet_grpc_client::{
    grpc::{GetBalanceRequest, GetBalanceResponse, TransactionEventRequest, TransactionEventResponse},
    WalletGrpcClient,
};
use tokio::{
    select,
    sync::mpsc::{self, error::TryRecvError},
    time::{interval, sleep, Duration},
};

pub struct WalletGrpc {
    _tx: mpsc::UnboundedSender<WalletAction>,
}

impl WalletGrpc {
    pub fn new(out_tx: mpsc::UnboundedSender<Reaction>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let worker = WalletGrpcWorker { rx, out_tx };
        tokio::spawn(worker.entrypoint());
        Self { _tx: tx }
    }
}

struct WalletGrpcWorker {
    rx: mpsc::UnboundedReceiver<WalletAction>,
    out_tx: mpsc::UnboundedSender<Reaction>,
}

impl WalletGrpcWorker {
    pub async fn entrypoint(mut self) {
        loop {
            let res = self.routine().await;
            let delta = WalletDelta::SetActive(false);
            self.send_update(delta).ok();
            if let Err(err) = res {
                log::error!("Wallet grpc routine failed: {}", err);
                // Drains the channel
                loop {
                    let action = self.rx.try_recv();
                    match action {
                        Ok(_) => {},
                        Err(TryRecvError::Disconnected) => {
                            return;
                        },
                        Err(TryRecvError::Empty) => {
                            break;
                        },
                    }
                }
                sleep(Duration::from_millis(3_000)).await;
            } else {
                break;
            }
        }
    }

    async fn routine(&mut self) -> Result<(), Error> {
        let mut client = WalletGrpcClient::connect("http://127.0.0.1:18143").await?;

        let request = TransactionEventRequest {};
        let mut transaction_events = client.stream_transaction_events(request).await?.into_inner();

        let delta = WalletDelta::SetActive(true);
        self.send_update(delta)?;

        let mut update = interval(Duration::from_millis(5_000));

        loop {
            select! {
                action = self.rx.recv() => {
                    if let Some(_action) = action {
                        // TODO: Process action
                    } else {
                        break;
                    }
                }
                event = transaction_events.next() => {
                    if let Some(response) = event.transpose()? {
                        self.process_transaction_response(response)?;
                    }
                }
                _ = update.tick() => {
                    let request = GetBalanceRequest {};
                    let response = client.get_balance(request).await?.into_inner();
                    self.process_balance(response)?;
                }
            }
        }
        Ok(())
    }

    fn process_balance(&mut self, response: GetBalanceResponse) -> Result<(), Error> {
        let balance = WalletBalance {
            available: response.available_balance,
            pending_incoming: response.pending_incoming_balance,
            pending_outgoing: response.pending_outgoing_balance,
        };
        let delta = WalletDelta::UpdateBalance(balance);
        self.send_update(delta)
    }

    fn process_transaction_response(&mut self, response: TransactionEventResponse) -> Result<(), Error> {
        if let Some(value) = response.transaction {
            let wt = WalletTransaction {
                event: value.event,
                tx_id: value.tx_id,
                // source_pk: value.source_pk,
                // dest_pk: value.dest_pk,
                status: value.status,
                direction: value.direction,
                amount: value.amount,
                message: value.message,
                is_coinbase: value.is_coinbase,
            };
            let delta = WalletDelta::LogTransaction(wt);
            self.send_update(delta)?;
        }
        Ok(())
    }

    fn send_update(&mut self, delta: WalletDelta) -> Result<(), Error> {
        let msg = Reaction::Delta(LaunchpadDelta::WalletDelta(delta));
        self.out_tx
            .send(msg)
            .map_err(|_| Error::msg("Can't send update for the wallet"))
    }
}
