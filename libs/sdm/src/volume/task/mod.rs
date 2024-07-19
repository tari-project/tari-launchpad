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

use super::ManagedVolume;
use crate::{
    config::ManagedProtocol,
    error::ParseError,
    task::{RunnableContext, RunnableTask, TaskContext, TaskStatusChecker},
    utils::TaskGuard,
};

pub struct VolumeTask<C: ManagedProtocol> {
    events: Option<TaskGuard<()>>,
    volume: Box<dyn ManagedVolume<Protocol = C>>,

    volume_name: String,
}

impl<C: ManagedProtocol> VolumeTask<C> {
    pub fn new(scope: &str, volume: Box<dyn ManagedVolume<Protocol = C>>) -> Self {
        let volume_name = format!("{}_{}", scope, volume.volume_name());
        Self {
            events: None,
            volume,
            volume_name,
        }
    }
}

#[async_trait]
impl<C: ManagedProtocol> RunnableTask for VolumeTask<C> {
    type Event = Event;
    type Protocol = C;
    type Status = Status;

    fn name(&self) -> &str {
        self.volume_name.as_ref()
    }

    fn is_permanent(&self) -> bool {
        true
    }
}

#[async_trait]
impl<C: ManagedProtocol> RunnableContext<VolumeTask<C>> for TaskContext<VolumeTask<C>> {
    async fn initialize(&mut self) {
        self.subscribe_to_events();
    }

    fn reconfigure(&mut self, config: Option<&C::Config>) -> bool {
        self.inner.volume.reconfigure(config)
    }

    fn process_inner_event(&mut self, _event: C::Inner) {
        log::warn!("Inner event is ignored by a volume task");
    }

    fn process_event(&mut self, event: Event) -> Result<(), Error> {
        self.process_event_impl(event)
    }

    async fn update(&mut self) -> Result<(), Error> {
        self.process_update_impl().await
    }

    fn is_active(&mut self) -> bool {
        match self.status.get() {
            Status::Active { .. } => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Status {
    InitialState,
    Checking,
    WaitCreating,
    Active,
}

impl TaskStatusChecker for Status {
    fn is_ready(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::InitialState
    }
}

#[derive(Debug)]
pub enum Event {
    Destroyed,
    Created,
}

impl TryFrom<String> for Event {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Docker values!
        match value.as_ref() {
            "destroy" => Ok(Self::Destroyed),
            "create" => Ok(Self::Created),
            _ => Err(ParseError(value)),
        }
    }
}
