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

use crate::{
    action::Do,
    address::{Address, SendError},
};

pub trait Sender<M>: Send {
    fn send(&self, msg: M) -> Result<(), SendError>;
}

impl<A, M> Sender<M> for Address<A>
where
    A: Do<M>,
    M: Send + 'static,
{
    fn send(&self, msg: M) -> Result<(), SendError> {
        Address::send(self, msg)
    }
}

pub struct Recipient<M> {
    sender: Box<dyn Sender<M>>,
}

impl<A, M> From<Address<A>> for Recipient<M>
where
    A: Do<M>,
    M: Send + 'static,
{
    fn from(address: Address<A>) -> Self {
        Self {
            sender: Box::new(address),
        }
    }
}

impl<M> Recipient<M> {
    pub fn send(&self, msg: M) -> Result<(), SendError> {
        self.sender.send(msg)
    }
}

pub struct Notifier<M> {
    message: M,
    sender: Box<dyn Sender<M>>,
}

impl<A, M> From<(Address<A>, M)> for Notifier<M>
where
    A: Do<M>,
    M: Send + 'static,
{
    fn from((address, message): (Address<A>, M)) -> Self {
        Self {
            message,
            sender: Box::new(address),
        }
    }
}

impl<M: Clone> Notifier<M> {
    pub fn notify(&self) -> Result<(), SendError> {
        let msg = self.message.clone();
        self.sender.send(msg)
    }
}
