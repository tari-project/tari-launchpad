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

use std::pin::Pin;

use anyhow::Error;
use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use futures::stream::{FusedStream, Stream, StreamExt};
use tari_launchpad_protocol::container::TaskProgress;
use tokio::{
    select,
    time::{sleep, Duration},
};

use super::task::Event;
use crate::{image::ManagedProtocol, scope::ControlEvent, task::TaskSender};

#[derive(Debug)]
pub enum CheckerEvent {
    Progress(TaskProgress),
    Ready,
}

pub struct CheckerContext<P: ManagedProtocol> {
    logs: Logs,
    sender: TaskSender<Event, P>,
}

impl<P: ManagedProtocol> CheckerContext<P> {
    pub(crate) fn new(logs: Logs, sender: TaskSender<Event, P>) -> Self {
        Self { logs, sender }
    }

    /// Reports the task about the progress.
    pub fn report(&self, event: CheckerEvent) -> Result<(), Error> {
        let event = Event::CheckerEvent(event);

        self.sender.send_direct(event)
    }

    /// Notifies all tasks with the inner event.
    pub fn notify(&self, event: P::Inner) -> Result<(), Error> {
        let event = ControlEvent::InnerEvent(event);

        self.sender.send_broadcast(event)
    }
}

#[async_trait]
pub trait ContainerChecker<P: ManagedProtocol>: Send {
    async fn entrypoint(mut self: Box<Self>, mut ctx: CheckerContext<P>) {
        let progress = TaskProgress::new("Starting...");
        ctx.report(CheckerEvent::Progress(progress)).ok();
        loop {
            select! {
                log_event = ctx.logs.next() => {
                    if let Some(Ok(msg)) = log_event {
                        self.on_log_event(&msg, &mut ctx).await;
                        ctx.sender.send_logs(msg).ok();
                    }
                }
                _ = sleep(Duration::from_secs(1)) => {
                    if let Err(err) = self.on_interval(&mut ctx).await {
                        log::error!("On interval checker failed: {}", err);
                    }
                }
            }
        }
    }

    async fn on_log_event(&mut self, _record: &str, _ctx: &mut CheckerContext<P>) {}

    async fn on_interval(&mut self, _ctx: &mut CheckerContext<P>) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ReadyIfStarted;

#[async_trait]
impl<P: ManagedProtocol> ContainerChecker<P> for ReadyIfStarted {
    async fn entrypoint(mut self: Box<Self>, ctx: CheckerContext<P>) {
        ctx.report(CheckerEvent::Ready).ok();
    }
}

#[derive(Deref, DerefMut)]
pub struct Logs {
    stream: Pin<Box<dyn FusedStream<Item = Result<String, Error>> + Send>>,
}

impl Logs {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<String, Error>>,
        S: Send + 'static,
    {
        Self {
            stream: Box::pin(stream.fuse()),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Stats {
    stream: Pin<Box<dyn FusedStream<Item = Result<(), Error>> + Send>>,
}

impl Stats {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<(), Error>>,
        S: Send + 'static,
    {
        Self {
            stream: Box::pin(stream.fuse()),
        }
    }
}
