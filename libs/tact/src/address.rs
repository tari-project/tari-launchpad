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

use thiserror::Error;
use tokio::sync::{mpsc, watch};

use crate::{
    action::{Do, Interrupt},
    actor::Actor,
    handler::Envelope,
    joint::ActorState,
};

#[derive(Debug, Error)]
#[error("Can't send an event to an actor")]
pub struct SendError;

pub struct Address<A: Actor> {
    tx_event: mpsc::UnboundedSender<Envelope<A>>,
    rx_state: watch::Receiver<ActorState>,
}

impl<A: Actor> Clone for Address<A> {
    fn clone(&self) -> Self {
        Self {
            tx_event: self.tx_event.clone(),
            rx_state: self.rx_state.clone(),
        }
    }
}

impl<A: Actor> Address<A> {
    pub(super) fn new(tx_event: mpsc::UnboundedSender<Envelope<A>>, rx_state: watch::Receiver<ActorState>) -> Self {
        Self { tx_event, rx_state }
    }

    pub fn send<E>(&self, event: E) -> Result<(), SendError>
    where
        A: Do<E>,
        E: Send + 'static,
    {
        let envelope = Envelope::from_event(event);
        self.tx_event.send(envelope).map_err(|_| SendError)
    }

    pub fn interrupt(&mut self) -> Result<(), SendError>
    where
        A: Do<Interrupt>,
    {
        self.send(Interrupt)
    }

    pub async fn join(&mut self) -> Result<(), SendError> {
        loop {
            let state = self.rx_state.borrow_and_update().clone();
            if state == ActorState::Finished {
                break;
            }
            self.rx_state.changed().await.map_err(|_| SendError)?;
        }
        Ok(())
    }
}
