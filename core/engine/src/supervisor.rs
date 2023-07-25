use crate::scope::Scope;
use anyhow::Error;
use async_trait::async_trait;
use bollard::Docker;
use tact::{Actor, ActorContext, Do};

pub struct Supervisor {}

impl Supervisor {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Actor for Supervisor {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        ctx.do_next(ReadConfig)?;
        ctx.do_next(SpawnScope)?;
        Ok(())
    }
}

struct ReadConfig;

#[async_trait]
impl Do<ReadConfig> for Supervisor {
    type Error = Error;

    async fn handle(
        &mut self,
        _: ReadConfig,
        _ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        log::info!("Reading configuration...");
        Ok(())
    }
}

struct SpawnScope;

#[async_trait]
impl Do<SpawnScope> for Supervisor {
    type Error = Error;

    async fn handle(
        &mut self,
        _: SpawnScope,
        _ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let scope = "tari_scope".to_string();
        log::info!("Spawning the scope: {}", scope);
        let docker = Docker::connect_with_local_defaults()?;
        let scope_task = Scope::new(docker, scope);
        scope_task.start();
        Ok(())
    }
}
