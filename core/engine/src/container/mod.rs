mod fsm;

use crate::docker::{DockerEvent, PullProgress};
use crate::types::{ManagedContainer, TaskProgress, TaskStatus};
use anyhow::Error;
use async_trait::async_trait;
use bollard::container::{Config, CreateContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::models::{CreateImageInfo, EventMessage, EventMessageTypeEnum};
use bollard::Docker;
use fsm::ContainerTaskFsm;
use tact::{Actor, ActorContext, Do, Receiver};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ImageInfo {
    scope: String,
    registry: String,
    image_name: String,
    tag: String,
}

/// A task that maintains a container and pulls an image
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContainerInfo {
    image_info: ImageInfo,
    /// The full image name
    image_name: String,
    container_name: String,
}

impl From<ImageInfo> for ContainerInfo {
    fn from(image_info: ImageInfo) -> ContainerInfo {
        let image_name = format!(
            "{}/{}:{}",
            image_info.registry, image_info.image_name, image_info.tag
        );
        let container_name = format!("{}_{}", image_info.scope, image_info.image_name);
        Self {
            image_info,
            image_name,
            container_name,
        }
    }
}

#[derive(Debug)]
enum Status {
    InitialState,

    PullingImage {
        progress_rx: Receiver,
    },

    CleanDangling,
    WaitContainerKilled,
    WaitContainerRemoved,

    CreateContainer,
    WaitContainerCreated,

    StartContainer,
    WaitContainerStarted,

    /// Check the `active` flag
    Idle,

    Active {
        // checker: Task,
        ready: bool,
    },

    DropImage,
}

#[derive(Debug)]
pub enum CheckerEvent {
    Progress(TaskProgress),
    Ready,
}

#[derive(Debug, Error)]
#[error("Can't parse value: {0}")]
pub struct ParseError(pub String);

#[derive(Debug)]
enum Event {
    Destroyed,
    PullingProgress(TaskProgress),
    Created,
    Started,
    Killed,
    Terminated,
    CheckerEvent(CheckerEvent),
}

impl TryFrom<String> for Event {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Docker values!
        match value.as_ref() {
            "destroy" => Ok(Self::Destroyed),
            "create" => Ok(Self::Created),
            "start" => Ok(Self::Started),
            "kill" => Ok(Self::Killed),
            "die" => Ok(Self::Terminated),
            _ => Err(ParseError(value)),
        }
    }
}

pub struct ContainerTask {
    docker: Docker,
    mc: Box<dyn ManagedContainer>,
    container_info: ContainerInfo,
    pull_progress: u8,
    events: Option<Receiver>,
    status: Status,
    task_status: TaskStatus,

    /// A flag to ask to restart a container
    force_restart: bool,
    /// A flag to drop and pull image again
    force_pull: bool,
}

impl ContainerTask {
    pub fn new(scope: String, docker: Docker, mc: impl ManagedContainer) -> Self {
        let image_info = ImageInfo {
            scope,
            registry: mc.registry().to_string(),
            image_name: mc.image_name().to_string(),
            tag: mc.tag().to_string(),
        };
        let container_info = ContainerInfo::from(image_info);
        Self {
            docker,
            mc: Box::new(mc),
            container_info,
            pull_progress: 0,
            events: None,
            status: Status::InitialState,
            task_status: TaskStatus::Inactive,
            force_restart: false,
            force_pull: false,
        }
    }

    fn image(&self) -> &str {
        &self.container_info.image_name
    }

    fn container(&self) -> &str {
        &self.container_info.container_name
    }

    fn should_be_active(&self) -> bool {
        // TODO: Check dependencies
        true
    }
}

#[async_trait]
impl Actor for ContainerTask {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        log::info!("Spawning a task to control the container: {}", self.image());
        let mut fsm = ContainerTaskFsm::new(self, ctx);
        fsm.subscribe_to_events();
        ctx.do_next(ProcessChanges)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),
    #[error("Type is empty")]
    TypeEmpty,
    #[error("Action is empty")]
    ActionEmpty,
    #[error("Actor is empty")]
    ActorEmpty,
    #[error("Can't parse the message: {0}")]
    ParseError(#[from] ParseError),
    #[error("Message for other container {actual}, but expected {expected}")]
    WrongImage { expected: String, actual: String },
    #[error("Process event error: {0}")]
    ProcessEventError(#[from] Error),
}

#[async_trait]
impl Do<DockerEvent> for ContainerTask {
    // TODO: Add custom error and the `fallback` method
    type Error = EventError;

    async fn handle(
        &mut self,
        msg: DockerEvent,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        log::debug!("Event from {}: {msg:?}", self.image());
        let image_name = self.image();
        let mut event: Option<Event> = None;
        let result = msg.result?;
        if let EventMessage {
            typ: Some(typ),
            action: Some(action),
            actor: Some(actor),
            ..
        } = result
        {
            if let Some(attributes) = actor.attributes {
                if let Some(name) = attributes.get("name") {
                    // TODO: Check images as well
                    if image_name == *name {
                        // TODO: Check the name
                        if let EventMessageTypeEnum::CONTAINER = typ {
                            event = Some(action.try_into()?);
                        }
                    } else {
                        return Err(EventError::WrongImage {
                            expected: image_name.to_string(),
                            actual: name.to_string(),
                        });
                    }
                }
            }
        }
        if let Some(event) = event {
            let mut fsm = ContainerTaskFsm::new(self, ctx);
            fsm.process_event(event)?;
        }
        Ok(())
    }
}

struct ProcessChanges;

#[async_trait]
impl Do<ProcessChanges> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        _: ProcessChanges,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let mut fsm = ContainerTaskFsm::new(self, ctx);
        fsm.process_changes().await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PullError {
    #[error("Docker error: {0}")]
    Bollard(#[from] bollard::errors::Error),
    #[error("Progress is empty")]
    ProgressEmpty,
    #[error("Current is empty")]
    CurrentEmpty,
    #[error("Total is empty")]
    TotalEmpty,
    #[error("Status is empty")]
    StatusEmpty,
}

#[async_trait]
impl Do<PullProgress> for ContainerTask {
    type Error = PullError;

    async fn handle(
        &mut self,
        msg: PullProgress,
        _ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let info = msg.result?;
        log::info!("Pulling info {}: {:?}", self.image(), info);
        let details = info.progress_detail.ok_or(PullError::ProgressEmpty)?;
        let current = details.current.ok_or(PullError::CurrentEmpty)? * 100;
        let total = details.total.ok_or(PullError::TotalEmpty)?;
        let pct = current / total;
        let _stage = info.status.ok_or(PullError::StatusEmpty)?;
        self.pull_progress = pct as u8;
        // TODO: Detect pulling is done
        // TODO: Report about the progress to the bus
        Ok(())
    }

    async fn fallback(
        &mut self,
        err: PullError,
        _ctx: &mut ActorContext<Self>,
    ) -> Result<(), Error> {
        log::error!("Can't pull image: {err}");
        // TODO: Handle pull errors
        // Restart pulling, etc...
        Ok(())
    }
}
