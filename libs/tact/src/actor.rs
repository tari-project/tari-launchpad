use anyhow::Error;
use async_trait::async_trait;

use crate::{address::Address, context::ActorContext, runtime::ActorRuntime};

#[async_trait]
pub trait Actor: Send + Sized + 'static {
    async fn initialize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        Ok(())
    }

    async fn finalize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        Ok(())
    }

    fn start(self) -> Address<Self> {
        let runtime = ActorRuntime::new(self);
        let address = runtime.context().address().clone();
        tokio::spawn(runtime.entyrpoint());
        address
    }
}
