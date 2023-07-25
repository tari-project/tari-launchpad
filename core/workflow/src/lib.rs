use anyhow::Error;
use async_trait::async_trait;
use tact::{Actor, ActorContext};
use tari_lp_bus::LpBusServer;

pub struct LpWorkflow {
    bus: LpBusServer,
}

impl LpWorkflow {
    pub fn new(bus: LpBusServer) -> Self {
        Self { bus }
    }
}

#[async_trait]
impl Actor for LpWorkflow {
    async fn initialize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        // TODO: Get a stream of messages
        Ok(())
    }
}
