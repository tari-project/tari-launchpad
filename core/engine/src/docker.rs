use bollard::errors::Error;
use bollard::models::{CreateImageInfo, EventMessage};
use derive_more::From;

#[derive(Debug, From)]
pub struct DockerEvent {
    pub result: Result<EventMessage, Error>,
}

#[derive(Debug, From)]
pub struct PullProgress {
    pub result: Result<CreateImageInfo, Error>,
}
