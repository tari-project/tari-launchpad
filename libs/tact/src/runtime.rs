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

use std::any::type_name;

use crate::{actor::Actor, context::ActorContext, joint::ActorState};

pub(super) struct ActorRuntime<A: Actor> {
    actor: A,
    context: ActorContext<A>,
}

impl<A: Actor> ActorRuntime<A> {
    pub fn new(actor: A) -> Self {
        let context = ActorContext::new();
        Self { actor, context }
    }

    pub async fn entyrpoint(mut self) {
        let name = type_name::<Self>();
        let res = self.actor.initialize(&mut self.context).await;
        if let Err(err) = res {
            log::error!("Actor {name} can't be initialized: {err}");
        }
        while let Some(envelope) = self.context.joint().recv().await {
            let handler = envelope.into_handler();
            let res = handler.handle(&mut self.actor, &mut self.context).await;
            if let Err(err) = res {
                log::error!("Actor {name} handler failed: {err}");
            }
        }
        let res = self.actor.finalize(&mut self.context).await;
        if let Err(err) = res {
            log::error!("Actor {name} can't be finalized: {err}",);
        }
        let res = self.context.joint().update_state(ActorState::Finished);
        if let Err(err) = res {
            log::error!("Actor {name} can't update the state: {err}");
        }
    }

    pub fn context(&self) -> &ActorContext<A> {
        &self.context
    }
}
