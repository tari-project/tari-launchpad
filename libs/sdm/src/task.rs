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

use std::{collections::HashMap, fmt};

use anyhow::Error;
use async_trait::async_trait;
use bollard::Docker;
use chrono::Local;
use derive_more::{Deref, DerefMut};
use futures::StreamExt;
use log::*;
use tari_launchpad_protocol::{
    container::{LogLevel, LogRecord, StatsData, TaskDelta, TaskId, TaskState, TaskStatus as TaskStatusValue},
    errors::ErrorRecord,
};
use tokio::{
    select,
    sync::{broadcast, mpsc},
    time::{sleep, Duration},
};
use tokio_stream::wrappers::{BroadcastStream, UnboundedReceiverStream};

use crate::{
    config::ManagedProtocol,
    scope::{ControlEvent, Report, ReportEnvelope},
    status::SdmStatus,
};

pub trait ManagedTask {
    fn id() -> TaskId;

    fn deps() -> Vec<TaskId> {
        Vec::default()
    }
}

pub trait TaskStatusChecker: fmt::Debug + Default + Send {
    fn is_ready(&self) -> bool {
        false
    }
}

#[async_trait]
pub trait RunnableTask: Sized + Send + 'static {
    type Protocol: ManagedProtocol;
    type Status: TaskStatusChecker;
    type Event: TaskEvent;

    fn name(&self) -> &str;

    /// Indicates whether any resources created by this task should persist between runs.
    fn is_permanent(&self) -> bool;
}

#[async_trait]
pub trait RunnableContext<T: RunnableTask> {
    /// Subscribe to events here
    async fn initialize(&mut self);
    fn reconfigure(&mut self, config: Option<&<T::Protocol as ManagedProtocol>::Config>) -> bool;
    fn process_inner_event(&mut self, event: <T::Protocol as ManagedProtocol>::Inner);
    fn process_event(&mut self, event: T::Event) -> Result<(), Error>;
    async fn update(&mut self) -> Result<(), Error>;
}

pub struct TaskSender<E, P: ManagedProtocol> {
    task_id: TaskId,
    event_tx: mpsc::UnboundedSender<E>,
    rep_tx: mpsc::UnboundedSender<ReportEnvelope<P>>,
    req_tx: broadcast::Sender<ControlEvent<P>>,
}

impl<E, P: ManagedProtocol> Clone for TaskSender<E, P> {
    fn clone(&self) -> Self {
        Self {
            task_id: self.task_id.clone(),
            event_tx: self.event_tx.clone(),
            rep_tx: self.rep_tx.clone(),
            req_tx: self.req_tx.clone(),
        }
    }
}

impl<E, P: ManagedProtocol> TaskSender<E, P> {
    pub fn get_direct(&self) -> &mpsc::UnboundedSender<E> {
        &self.event_tx
    }

    pub fn send_direct(&self, event: E) -> Result<(), Error> {
        self.event_tx
            .send(event)
            .map_err(|_| Error::msg("Can't send a direct message"))
    }

    // pub fn get_broadcast(&self) -> &broadcast::Sender<ControlEvent<P>> {
    // &self.req_tx
    // }

    pub fn send_broadcast(&self, event: ControlEvent<P>) -> Result<(), Error> {
        self.req_tx
            .send(event)
            .map(drop)
            .map_err(|_| Error::msg("Can't send a direct message"))
    }

    // pub fn get_reporter(&self) -> &mpsc::UnboundedSender<ReportEnvelope<P>> {
    // &self.rep_tx
    // }

    pub fn send_report(&self, report: Report<P>) -> Result<(), Error> {
        let envelope = ReportEnvelope {
            task_id: self.task_id.clone(),
            details: report,
        };
        self.rep_tx
            .send(envelope)
            .map_err(|_| Error::msg("Can't send a report"))
    }

    pub fn send_logs(&self, message: String) -> Result<(), Error> {
        let record = LogRecord {
            datetime: Local::now().naive_local(),
            // TODO: Detect the real level
            level: LogLevel::Info,
            message,
        };
        let delta = TaskDelta::LogRecord(record);
        let report = Report::Delta(delta);
        self.send_report(report)
    }

    pub fn send_stats(&self, record: StatsData) -> Result<(), Error> {
        let delta = TaskDelta::StatsRecord(record);
        let report = Report::Delta(delta);
        self.send_report(report)
    }

    pub fn send_error(&self, message: String) -> Result<(), Error> {
        let record = ErrorRecord {
            datetime: Local::now().naive_local(),
            message,
        };
        let delta = TaskDelta::LogError(record);
        let report = Report::Delta(delta);
        self.send_report(report)
    }
}

#[derive(Deref, DerefMut)]
pub struct TaskContext<T: RunnableTask> {
    /// Filled by a dependencies controller
    dependencies_ready: bool,
    resources_map: HashMap<TaskId, String>,
    /// Depends on the config
    should_start: bool,
    pub status: SdmStatus<T::Status>,
    sender: TaskSender<T::Event, T::Protocol>,
    pub driver: Docker,
    #[deref]
    #[deref_mut]
    pub inner: T,
}

impl<T: RunnableTask> TaskContext<T> {
    pub fn should_be_active(&self) -> bool {
        self.should_start && self.dependencies_ready
    }

    pub fn resource(&self, id: &TaskId) -> Option<&str> {
        self.resources_map.get(id).map(String::as_ref)
    }

    pub fn sender(&self) -> &TaskSender<T::Event, T::Protocol> {
        &self.sender
    }

    pub fn update_task_status(&self, status: TaskStatusValue) -> Result<(), Error> {
        let delta = TaskDelta::UpdateStatus(status);
        let report = Report::Delta(delta);
        self.sender().send_report(report)
    }
}

pub struct SdmTaskRunner<R: RunnableTask> {
    task_id: TaskId,
    events_receiver: Option<mpsc::UnboundedReceiver<R::Event>>,
    requests_receiver: Option<broadcast::Receiver<ControlEvent<R::Protocol>>>,
    requests_sender: broadcast::Sender<ControlEvent<R::Protocol>>,
    context: TaskContext<R>,
    /// Waits when these dependencies started.
    dependencies: HashMap<TaskId, bool>,
    ready_to_use: bool,
}

impl<R: RunnableTask> SdmTaskRunner<R>
where
    TaskContext<R>: RunnableContext<R>,
{
    pub fn new<M: ManagedTask>(
        req_tx: broadcast::Sender<ControlEvent<R::Protocol>>,
        rep_tx: mpsc::UnboundedSender<ReportEnvelope<R::Protocol>>,
        inner: R,
        docker: Docker,
    ) -> Self {
        let task_id = M::id();
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let sender = TaskSender {
            task_id: task_id.clone(),
            event_tx,
            rep_tx,
            req_tx: req_tx.clone(),
        };
        let context = TaskContext {
            dependencies_ready: false,
            resources_map: HashMap::new(),
            should_start: false,
            status: SdmStatus::new(inner.name().to_string()),
            sender,
            driver: docker,
            inner,
        };
        // It subscribed here to avoid the gap if that will subscribe in the routine.
        let req_rx = req_tx.subscribe();
        let dependencies = M::deps().into_iter().map(|id| (id, false)).collect();
        Self {
            // TODO: Consider to use `task_id` from a sender
            task_id,
            events_receiver: Some(event_rx),
            requests_receiver: Some(req_rx),
            requests_sender: req_tx,
            context,
            dependencies,
            ready_to_use: false,
        }
    }

    pub async fn entrypoint(mut self) {
        if let Err(err) = self.routine().await {
            error!("Task failed: {}", err);
        }
    }

    pub async fn routine(&mut self) -> Result<(), Error> {
        self.check_dependencies();
        self.initialize().await?;
        let interval = Duration::from_millis(1_000);
        let events_receiver = self.events_receiver.take().unwrap();
        let mut events = UnboundedReceiverStream::new(events_receiver);
        let requests_receiver = self.requests_receiver.take().unwrap();
        let mut requests = BroadcastStream::new(requests_receiver);
        loop {
            select! {
                _ = sleep(interval) => {
                    trace!(
                        "[Routine loop] !{}::update={:?} ... interval",
                        self.context.name(),
                        self.context.status.get()
                    );
                    // log::trace!("Checking the scope by interval");
                }
                event = events.next() => {
                    trace!(
                        "[Routine loop] !{}::update={:?} ... events",
                        self.context.name(),
                        self.context.status.get()
                    );
                    if let Some(event) = event {
                        self.process_event(event);
                    } else {
                        log::info!("Events stream closed");
                        break;
                    }
                }
                req = requests.next() => {
                    trace!(
                        "[Routine loop] !{}::update={:?} ... requests",
                        self.context.name(),
                        self.context.status.get()
                    );
                    if let Some(Ok(req)) = req {
                        self.process_request(req);
                    } else {
                        log::info!("Requests stream closed");
                        break;
                    }
                }
            }
            self.update().await;
            self.notify_dependants();
        }
        Ok(())
    }

    fn notify_dependants(&mut self) {
        if self.context.status.is_ready() {
            // Notifies dependants about the entity is ready to use
            if !self.ready_to_use {
                self.ready_to_use = true;
                let task_id = self.task_id.clone();
                let name = self.context.name().to_owned();
                let event = ControlEvent::ResourceReady { task_id, name };
                self.broadcast(event);
            }
        } else {
            // Notifies dependants about the entity is not ready anymore
            if self.ready_to_use {
                self.ready_to_use = false;
                let task_id = self.task_id.clone();
                let event = ControlEvent::ResourceClosed { task_id };
                self.broadcast(event);
            }
        }
    }

    fn broadcast(&mut self, event: ControlEvent<R::Protocol>) {
        if let Err(err) = self.requests_sender.send(event) {
            log::error!("Can't broadcast event: {:?}", err);
        }
    }

    fn process_request(&mut self, req: ControlEvent<R::Protocol>) {
        match req {
            ControlEvent::SetConfig(config) => {
                let config = config.as_deref();
                self.reconfigure(config);
            },
            ControlEvent::ResourceReady { task_id, name } => {
                if let Some(flag) = self.dependencies.get_mut(&task_id) {
                    *flag = true;
                    self.context.resources_map.insert(task_id, name);
                    // Check dependencies if any flag changed
                    self.check_dependencies();
                }
            },
            ControlEvent::ResourceClosed { task_id } => {
                if let Some(flag) = self.dependencies.get_mut(&task_id) {
                    *flag = false;
                    self.context.resources_map.remove(&task_id);
                    // Check dependencies if any flag changed
                    self.check_dependencies();
                }
            },
            ControlEvent::InnerEvent(inner) => {
                self.process_inner_event(inner);
            },
        }
    }

    /// Checks whether all dependencies have a ready status and sets the `dependencies_ready` field in the context
    /// accordingly.. Does not trigger any actions.
    fn check_dependencies(&mut self) {
        // If the set is empty `all` returns `true`.
        self.context.dependencies_ready = self.dependencies.values().all(|ready| *ready);
    }

    pub async fn initialize(&mut self) -> Result<(), Error> {
        self.context.initialize().await;
        let permanent = self.context.inner.is_permanent();
        let state = TaskState::new(permanent);
        let report = Report::State(state);
        self.context.sender.send_report(report)?;
        Ok(())
    }

    pub fn reconfigure(&mut self, config: Option<&<R::Protocol as ManagedProtocol>::Config>) {
        let active = self.context.reconfigure(config);
        if active {
            debug!("[SdmTaskRunner::reconfigure] Task {} is queued to start", self.task_id)
        } else {
            debug!("[SdmTaskRunner::reconfigure] Task {} is will NOT start", self.task_id)
        }
        self.context.should_start = active;
    }

    pub fn process_inner_event(&mut self, event: <R::Protocol as ManagedProtocol>::Inner) {
        self.context.process_inner_event(event);
    }

    pub fn process_event(&mut self, event: R::Event) {
        trace!("Processing event !{}::event={:?}", self.context.name(), event);
        if let Err(err) = self.context.process_event(event) {
            log::error!("Event processing error: {}", err);
        }
    }

    pub async fn update(&mut self) {
        loop {
            trace!(
                "[Update event] !{}::update={:?} ... update(loop entry)",
                self.context.name(),
                self.context.status.get()
            );
            self.context.status.check_fallback();
            self.context.status.reset_has_work_flag();
            if let Err(err) = self.context.update().await {
                error!("Update error: {}", err);
                break;
            }
            if !self.context.status.has_work() {
                break;
            }
        }
    }
}

pub trait TaskEvent: fmt::Debug + Send {}
