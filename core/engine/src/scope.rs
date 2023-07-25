use crate::container::ContainerTask;
use crate::images::Tor;
use anyhow::Error;
use async_trait::async_trait;
use bollard::Docker;
use tact::{Actor, ActorContext, Do};

pub struct Scope {
    docker: Docker,
    scope: String,
}

impl Scope {
    pub fn new(docker: Docker, scope: String) -> Self {
        Self { docker, scope }
    }
}

#[async_trait]
impl Actor for Scope {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        ctx.do_next(SpawnTasks)?;
        Ok(())
    }
}

struct SpawnTasks;

#[async_trait]
impl Do<SpawnTasks> for Scope {
    type Error = Error;

    async fn handle(
        &mut self,
        _: SpawnTasks,
        _ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        log::debug!("Spawning containers in the scope: {}", self.scope);
        let tor_task = ContainerTask::new(self.scope.clone(), self.docker.clone(), Tor);
        tor_task.start();
        Ok(())
    }
}
