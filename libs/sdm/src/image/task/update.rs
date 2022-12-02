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

use anyhow::Error;
use tari_launchpad_protocol::container::{TaskProgress, TaskStatus};

use super::{ContainerState, ImageTask, Status};
use crate::{config::ManagedProtocol, task::TaskContext};

impl<C: ManagedProtocol> TaskContext<ImageTask<C>> {
    pub async fn process_update_impl(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::InitialState => self.do_initial_state().await,
            Status::PullingImage { .. } => self.do_pulling().await,
            Status::CleanDangling => self.do_clean_dangling().await,
            Status::WaitContainerKilled => self.do_wait_container_killed().await,
            Status::WaitContainerRemoved => self.do_wait_container_removed().await,
            Status::Idle => self.do_idle().await,
            Status::CreateContainer => self.do_create_container().await,
            Status::WaitContainerCreated => self.do_wait_container_created().await,
            Status::StartContainer => self.do_start_container().await,
            Status::WaitContainerStarted => self.do_wait_container_started().await,
            Status::Active { .. } => self.do_active().await,
        }
    }

    async fn do_initial_state(&mut self) -> Result<(), Error> {
        self.update_task_status(TaskStatus::Inactive)?;

        log::debug!("Cheking image {} ...", self.inner.image_name);
        if self.image_exists().await {
            log::debug!("Image {} exists. Skip pulling.", self.inner.image_name);
            let progress = TaskProgress::new("Cleaning...");
            self.update_task_status(TaskStatus::Progress(progress))?;
            self.status.set(Status::CleanDangling);
        } else {
            log::debug!("Image {} doesn't exist. Pulling.", self.inner.image_name);
            let progress = TaskProgress::new("Pulling...");
            self.update_task_status(TaskStatus::Progress(progress))?;
            let progress = self.pull();
            self.status.set(Status::PullingImage { progress });
        }
        Ok(())
    }

    async fn do_pulling(&mut self) -> Result<(), Error> {
        if self.image_exists().await {
            // Just loaded, container can't be exist
            self.status.set(Status::Idle);
            self.update_task_status(TaskStatus::Inactive)?;
        }
        Ok(())
    }

    async fn do_clean_dangling(&mut self) -> Result<(), Error> {
        log::debug!("Cheking container {} ...", self.inner.container_name);
        let state = self.container_state().await;
        match state {
            ContainerState::Running => {
                log::debug!("Container {} is running. Terminating it.", self.inner.container_name);
                self.try_kill_container().await?;
                self.status.set(Status::WaitContainerKilled);
            },
            ContainerState::NotRunning => {
                log::debug!("Container {} is not running. Removing it.", self.inner.container_name);
                self.try_remove_container().await?;
                self.status.set(Status::WaitContainerRemoved);
            },
            ContainerState::NotFound => {
                log::debug!("Container {} doesn't exist.", self.inner.container_name);
                self.status.set(Status::Idle);
                self.update_task_status(TaskStatus::Inactive)?;
            },
        }
        Ok(())
    }

    async fn do_wait_container_killed(&mut self) -> Result<(), Error> {
        // TODO: Wait interval
        Ok(())
    }

    async fn do_wait_container_removed(&mut self) -> Result<(), Error> {
        // TODO: Wait interval
        Ok(())
    }

    async fn do_idle(&mut self) -> Result<(), Error> {
        if self.should_be_active() {
            log::debug!("Preparing a container {} to start...", self.inner.container_name);
            self.status.set(Status::CreateContainer);
            self.update_task_status(TaskStatus::Pending)?;
        }
        Ok(())
    }

    async fn do_create_container(&mut self) -> Result<(), Error> {
        log::debug!("Trying to create container {} ...", self.inner.container_name);
        self.try_create_container().await?;
        self.status.set(Status::WaitContainerCreated);
        Ok(())
    }

    async fn do_wait_container_created(&mut self) -> Result<(), Error> {
        // TODO: Check timeout
        Ok(())
    }

    async fn do_start_container(&mut self) -> Result<(), Error> {
        self.try_start_container().await?;
        self.status.set(Status::WaitContainerStarted);
        self.update_task_status(TaskStatus::Pending)?;
        Ok(())
    }

    async fn do_active(&mut self) -> Result<(), Error> {
        // TODO: Spawn `ready to use` worker

        // TODO: Remove the following
        if !self.should_be_active() {
            self.status.set(Status::CleanDangling);
        }
        Ok(())
    }

    async fn do_wait_container_started(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
