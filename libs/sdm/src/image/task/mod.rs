// Copyright 2022. The Tari Project
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

mod docker;
mod events;
mod update;

use anyhow::Error;
use async_trait::async_trait;
use tari_launchpad_protocol::container::TaskProgress;

use super::{checker::CheckerEvent, ManagedContainer};
use crate::{
    config::ManagedProtocol,
    error::ParseError,
    task::{RunnableContext, RunnableTask, TaskContext, TaskStatusChecker},
    utils::TaskGuard,
};

pub struct ImageTask<C: ManagedProtocol> {
    events: Option<TaskGuard<()>>,
    container_name: String,
    // TODO: Rename to `fqdn`
    image_name: String,
    image: Box<dyn ManagedContainer<Protocol = C>>,
    /// A flag to ask to restart a container
    force_restart: bool,
    /// A flag to drop and pull image again
    force_pull: bool,
}

impl<C: ManagedProtocol> ImageTask<C> {
    pub fn new(scope: &str, image: Box<dyn ManagedContainer<Protocol = C>>) -> Self {
        // let required = image.deps().into_iter().collect();
        let image_name = format!("{}/{}:{}", image.registry(), image.image_name(), image.tag());
        let container_name = format!("{}_{}", scope, image.image_name());
        Self {
            events: None,
            container_name,
            image_name,
            image,
            force_restart: false,
            force_pull: false,
        }
    }
}

#[async_trait]
impl<C: ManagedProtocol> RunnableTask for ImageTask<C> {
    type Event = Event;
    type Protocol = C;
    type Status = Status;

    fn name(&self) -> &str {
        self.container_name.as_ref()
    }

    fn is_permanent(&self) -> bool {
        false
    }
}

#[async_trait]
impl<C: ManagedProtocol> RunnableContext<ImageTask<C>> for TaskContext<ImageTask<C>> {
    async fn initialize(&mut self) {
        self.subscribe_to_events();
    }

    fn reconfigure(&mut self, config: Option<&C::Config>) -> bool {
        self.inner.image.reconfigure(config).unwrap_or_default()
    }

    fn process_inner_event(&mut self, event: C::Inner) {
        self.inner.image.on_event(event)
    }

    fn process_event(&mut self, event: Event) -> Result<(), Error> {
        self.process_event_impl(event)
    }

    async fn update(&mut self) -> Result<(), Error> {
        self.process_update_impl().await
    }
}

impl<C: ManagedProtocol> TaskContext<ImageTask<C>> {
    fn should_be_restarted(&self) -> bool {
        self.force_restart || self.force_pull
    }
}

#[derive(Debug)]
pub enum Status {
    InitialState,

    PullingImage {
        progress: TaskGuard<()>,
    },

    CleanDangling,
    WaitContainerKilled,
    WaitContainerRemoved,
    CannotStart,

    CreateContainer,
    WaitContainerCreated,

    StartContainer,
    WaitContainerStarted,

    /// Check the `active` flag
    Idle,

    Active {
        checker: TaskGuard<()>,
        ready: bool,
    },

    DropImage,
}

impl TaskStatusChecker for Status {
    fn is_ready(&self) -> bool {
        matches!(self, Self::Active { ready: true, .. })
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::InitialState
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerState {
    Empty,
    Created,
    Running,
    Paused,
    Restarting,
    Removing,
    Exited,
    Dead,
    ErrorStatusNotDefined,
    ErrorStateNotDefined,
    NotFound,
}

#[derive(Debug)]
pub enum Event {
    Destroyed,
    PullingProgress(TaskProgress),
    PullingFailed(String),
    Created,
    Started,
    Killed,
    Terminated,
    CheckerProgress(CheckerEvent),
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
