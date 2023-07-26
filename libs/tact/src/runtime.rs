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
