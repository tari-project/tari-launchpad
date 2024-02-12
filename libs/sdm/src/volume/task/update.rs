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

use super::{Status, VolumeTask};
use crate::{config::ManagedProtocol, task::TaskContext};

impl<C: ManagedProtocol> TaskContext<VolumeTask<C>> {
    pub async fn process_update_impl(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::InitialState => self.do_initial_state().await,
            Status::Checking => self.do_checking().await,
            Status::WaitCreating => self.do_wait_creating().await,
            Status::Active => self.do_active().await,
        }
    }

    async fn do_initial_state(&mut self) -> Result<(), Error> {
        log::trace!("[Update event: Volume] `do_initial_state`");
        self.update_task_status(TaskStatus::Inactive)?;
        self.status.set(Status::Checking);
        Ok(())
    }

    async fn do_checking(&mut self) -> Result<(), Error> {
        log::trace!("[Update event: Volume] `do_checking`");
        if self.volume_exists().await {
            self.status.set(Status::Active);
            self.update_task_status(TaskStatus::Active)?;
        } else {
            self.try_create_volume().await?;
            self.status.set(Status::WaitCreating);
            self.update_task_status(TaskStatus::Pending)?;
        }
        Ok(())
    }

    async fn do_wait_creating(&mut self) -> Result<(), Error> {
        log::trace!("[Update event: Volume] `do_wait_creating`");
        Ok(())
    }

    async fn do_active(&mut self) -> Result<(), Error> {
        log::trace!("[Update event: Volume] `do_active`");
        if !self.should_be_active() {
            // self.status.set(Status::Checking);
        }
        Ok(())
    }
}
