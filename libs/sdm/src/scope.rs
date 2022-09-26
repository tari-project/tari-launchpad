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

use std::sync::Arc;

use anyhow::{anyhow, Error};
use bollard::Docker;
use tari_launchpad_protocol::container::{TaskDelta, TaskId};
use tokio::sync::{broadcast, mpsc};

use crate::{
    config::ManagedProtocol,
    image::{ImageTask, ManagedContainer},
    network::{ManagedNetwork, NetworkTask},
    task::{ManagedTask, SdmTaskRunner},
    volume::{ManagedVolume, VolumeTask},
};

#[derive(Debug)]
pub struct ReportEnvelope<C: ManagedProtocol> {
    pub task_id: TaskId,
    pub details: Report<C>,
}

#[derive(Debug)]
pub enum Report<C: ManagedProtocol> {
    Delta(TaskDelta),
    Extras(C::Outer),
}

pub struct SdmScope<C: ManagedProtocol> {
    scope: String,
    docker: Docker,
    reporter: mpsc::UnboundedReceiver<ReportEnvelope<C>>,
    report_sender: mpsc::UnboundedSender<ReportEnvelope<C>>,
    sender: broadcast::Sender<ControlEvent<C>>,
}

// TODO: Move to the `task` mod?
#[derive(Debug)]
pub enum ControlEvent<C: ManagedProtocol> {
    SetConfig(Option<Arc<C::Config>>),
    ResourceReady {
        task_id: TaskId,
        /// id or name in the docker
        name: String,
    },
    ResourceClosed {
        task_id: TaskId,
    },
    InnerEvent(C::Inner),
}

impl<C: ManagedProtocol> Clone for ControlEvent<C> {
    fn clone(&self) -> Self {
        match self {
            Self::SetConfig(config) => Self::SetConfig(config.clone()),
            Self::ResourceReady { task_id, name } => Self::ResourceReady {
                task_id: task_id.clone(),
                name: name.clone(),
            },
            Self::ResourceClosed { task_id } => Self::ResourceClosed {
                task_id: task_id.clone(),
            },
            Self::InnerEvent(inner) => Self::InnerEvent(inner.clone()),
        }
    }
}

impl<C: ManagedProtocol> SdmScope<C> {
    pub fn connect(scope: &str) -> Result<Self, Error> {
        let docker = Docker::connect_with_local_defaults()?;
        // TODO: Use `rx` later to control entries
        let (req_tx, _req_rx) = broadcast::channel(16);
        let (rep_tx, rep_rx) = mpsc::unbounded_channel();
        Ok(Self {
            scope: scope.to_string(),
            docker,
            reporter: rep_rx,
            report_sender: rep_tx,
            sender: req_tx,
        })
    }

    pub fn add_image<I>(&mut self, entry: I) -> Result<(), Error>
    where I: ManagedContainer<Protocol = C> + ManagedTask {
        // TODO: DRY!
        let entry = Box::new(entry);
        let inner = ImageTask::new(&self.scope, entry);
        let runner = SdmTaskRunner::new::<I>(
            self.sender.clone(),
            self.report_sender.clone(),
            inner,
            self.docker.clone(),
        );
        tokio::spawn(runner.entrypoint());
        Ok(())
    }

    pub fn add_network<N>(&mut self, entry: N) -> Result<(), Error>
    where N: ManagedNetwork<Protocol = C> + ManagedTask {
        // TODO: DRY!
        let entry = Box::new(entry);
        let inner = NetworkTask::new(&self.scope, entry);
        let runner = SdmTaskRunner::new::<N>(
            self.sender.clone(),
            self.report_sender.clone(),
            inner,
            self.docker.clone(),
        );
        tokio::spawn(runner.entrypoint());
        Ok(())
    }

    pub fn add_volume<V>(&mut self, entry: V) -> Result<(), Error>
    where V: ManagedVolume<Protocol = C> + ManagedTask {
        // TODO: DRY!
        let entry = Box::new(entry);
        let inner = VolumeTask::new(&self.scope, entry);
        let runner = SdmTaskRunner::new::<V>(
            self.sender.clone(),
            self.report_sender.clone(),
            inner,
            self.docker.clone(),
        );
        tokio::spawn(runner.entrypoint());
        Ok(())
    }

    pub fn set_config(&mut self, config: Option<C::Config>) -> Result<(), Error> {
        let config = config.map(Arc::new);
        let req = ControlEvent::SetConfig(config);
        self.send(req)
    }

    fn send(&mut self, req: ControlEvent<C>) -> Result<(), Error> {
        self.sender
            .send(req)
            .map(drop)
            .map_err(|req| anyhow!("Can't send a request: {:?}", req))
    }

    pub async fn recv(&mut self) -> Option<ReportEnvelope<C>> {
        self.reporter.recv().await
    }

    pub fn stop(&self) {}
}
