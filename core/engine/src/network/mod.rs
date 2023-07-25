mod fsm;

use crate::docker::DockerEvent;
use crate::types::ManagedNetwork;
use anyhow::Error;
use async_trait::async_trait;
use bollard::models::EventMessage;
use bollard::Docker;
use derive_more::From;
use fsm::NetworkTaskFsm;
use tact::{Actor, ActorContext, Do, Receiver};

pub struct NetworkTask {
    docker: Docker,
    network_name: String,
    events: Option<Receiver>,
}

impl NetworkTask {
    pub fn new(scope: String, docker: Docker, mn: impl ManagedNetwork) -> Self {
        let network_name = format!("{}_{}", scope, mn.network_name());
        Self {
            docker,
            network_name,
            events: None,
        }
    }

    fn network(&self) -> &str {
        &self.network_name
    }
}

#[async_trait]
impl Actor for NetworkTask {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        log::info!("Spawning a task to control the network: {}", self.network());
        let mut fsm = NetworkTaskFsm::new(self, ctx);
        fsm.subscribe_to_events();
        // ctx.do_next(ProcessChanges)?;
        Ok(())
    }
}

#[async_trait]
impl Do<DockerEvent> for NetworkTask {
    type Error = Error;

    async fn handle(
        &mut self,
        msg: DockerEvent,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        Err(Error::msg("network messages not processed (TODO)"))
    }
}
