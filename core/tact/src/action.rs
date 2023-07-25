use anyhow::Error;
use async_trait::async_trait;

use crate::{actor::Actor, context::ActorContext};

#[async_trait]
pub trait Do<E>: Actor {
    type Error: Send + Into<Error> + 'static;

    async fn handle(&mut self, action: E, ctx: &mut ActorContext<Self>) -> Result<(), Self::Error>;

    async fn fallback(&mut self, err: Self::Error, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        Err(err.into())
    }
}

pub struct Interrupt;
