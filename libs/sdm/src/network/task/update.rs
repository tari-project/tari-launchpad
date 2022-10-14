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
use tari_launchpad_protocol::container::TaskStatus;

use super::{NetworkTask, Status};
use crate::{config::ManagedProtocol, task::TaskContext};

impl<C: ManagedProtocol> TaskContext<NetworkTask<C>> {
    pub async fn process_update_impl(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::InitialState => self.do_initial_state().await,
            Status::Cleanup => self.do_cleanup().await,
            Status::WaitRemoving => self.do_wait_removing().await,
            Status::Inactive => self.do_inactive().await,
            Status::WaitCreating => self.do_wait_creating().await,
            Status::Active => self.do_active().await,
        }
    }

    async fn do_initial_state(&mut self) -> Result<(), Error> {
        self.update_task_status(TaskStatus::Inactive)?;
        self.status.set(Status::Cleanup);
        Ok(())
    }

    async fn do_cleanup(&mut self) -> Result<(), Error> {
        if self.network_exists().await {
            self.try_remove_network().await?;
            self.status.set(Status::WaitRemoving);
            self.update_task_status(TaskStatus::Pending)?;
        } else {
            self.status.set(Status::Inactive);
            self.update_task_status(TaskStatus::Inactive)?;
        }
        Ok(())
    }

    async fn do_wait_removing(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn do_inactive(&mut self) -> Result<(), Error> {
        if self.should_be_active() {
            self.try_create_network().await?;
            self.status.set(Status::WaitCreating);
            self.update_task_status(TaskStatus::Pending)?;
        }
        Ok(())
    }

    async fn do_wait_creating(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn do_active(&mut self) -> Result<(), Error> {
        if !self.should_be_active() {
            self.status.set(Status::Cleanup);
        }
        Ok(())
    }
}
