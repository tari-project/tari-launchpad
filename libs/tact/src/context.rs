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

use tokio::sync::{mpsc, watch};

use crate::{
    action::Do,
    actor::Actor,
    address::{Address, SendError},
    joint::{ActorState, AddressJoint},
    recipient::{Notifier, Recipient},
};

pub struct ActorContext<A: Actor> {
    address: Address<A>,
    joint: AddressJoint<A>,
}

impl<A: Actor> ActorContext<A> {
    pub(super) fn new() -> Self {
        let (tx_event, rx_event) = mpsc::unbounded_channel();
        let (tx_state, rx_state) = watch::channel(ActorState::Active);
        let joint = AddressJoint::new(rx_event, tx_state);
        let address = Address::new(tx_event, rx_state);
        Self { address, joint }
    }

    pub fn address(&self) -> &Address<A> {
        &self.address
    }

    pub fn recipient<M>(&self) -> Recipient<M>
    where
        A: Do<M>,
        M: Send + 'static,
    {
        self.address.clone().into()
    }

    pub fn notifier<M>(&self, msg: M) -> Notifier<M>
    where
        A: Do<M>,
        M: Send + 'static,
    {
        (self.address.clone(), msg).into()
    }

    pub(crate) fn joint(&mut self) -> &mut AddressJoint<A> {
        &mut self.joint
    }

    pub fn do_next<E>(&self, action: E) -> Result<(), SendError>
    where
        A: Do<E>,
        E: Send + 'static,
    {
        self.address.send(action)
    }

    pub fn shutdown(&mut self) {
        self.joint.close();
    }
}
