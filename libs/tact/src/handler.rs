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
use async_trait::async_trait;

use crate::{action::Do, actor::Actor, context::ActorContext};

pub struct Envelope<A: Actor> {
    handler: Box<dyn Handler<A>>,
}

impl<A: Actor> Envelope<A> {
    pub(crate) fn into_handler(self) -> Box<dyn Handler<A>> {
        self.handler
    }

    pub fn from_event<E>(event: E) -> Self
    where
        A: Do<E>,
        E: Send + 'static,
    {
        let handler = HandlerImpl { event };
        Self {
            handler: Box::new(handler),
        }
    }
}

#[async_trait]
pub(crate) trait Handler<A: Actor>: Send {
    async fn handle(self: Box<Self>, actor: &mut A, ctx: &mut ActorContext<A>) -> Result<(), Error>;
}

struct HandlerImpl<E> {
    event: E,
}

#[async_trait]
impl<A: Do<E>, E: Send> Handler<A> for HandlerImpl<E> {
    async fn handle(self: Box<Self>, actor: &mut A, ctx: &mut ActorContext<A>) -> Result<(), Error> {
        if let Err(err) = actor.handle(self.event, ctx).await {
            actor.fallback(err, ctx).await
        } else {
            Ok(())
        }
    }
}
