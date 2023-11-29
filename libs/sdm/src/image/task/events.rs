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

use super::{Event, ImageTask, Status};
use crate::{
    config::ManagedProtocol,
    image::checker::{CheckerContext, CheckerEvent},
    task::TaskContext,
};

impl<C: ManagedProtocol> TaskContext<ImageTask<C>> {
    pub fn process_event_impl(&mut self, event: Event) -> Result<(), Error> {
        log::trace!("Image event triggered. Image: {} Event: {event:?}", self.image_name);
        match event {
            Event::Created => self.on_created(),
            Event::PullingProgress(value) => self.on_pulling_progress(value),
            Event::PullingFailed(reason) => self.on_pulling_failed(reason),
            Event::Destroyed => self.on_destroyed(),
            Event::Started => self.on_started(),
            Event::Killed => self.on_killed(),
            Event::Terminated => self.on_terminated(),
            Event::CheckerEvent(event) => self.on_checker_event(event),
        }
    }

    fn on_created(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerCreated = self.status.get() {
            self.status.set(Status::StartContainer);
        }
        Ok(())
    }

    fn on_pulling_progress(&mut self, value: TaskProgress) -> Result<(), Error> {
        if let Status::PullingImage { .. } = self.status.get() {
            self.update_task_status(TaskStatus::Progress(value))?;
        }
        Ok(())
    }

    fn on_pulling_failed(&mut self, reason: String) -> Result<(), Error> {
        if let Status::PullingImage { .. } = self.status.get() {
            self.status.set(Status::CannotStart);
            self.update_task_status(TaskStatus::Failed(reason))?;
        }
        Ok(())
    }

    fn on_destroyed(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerRemoved = self.status.get() {
            self.status.set(Status::CleanDangling);
        }
        Ok(())
    }

    fn on_started(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerStarted { .. } = self.status.get() {
            let checker = self.inner.image.checker();
            let logs = self.logs_stream();
            let stats = self.stats_stream();
            let sender = self.sender().clone();
            let context = CheckerContext::new(logs, stats, sender);
            let fur = checker.entrypoint(context);
            let checker = tokio::spawn(fur).into();
            self.status.set(Status::Active { checker, ready: false });
        }
        Ok(())
    }

    fn on_killed(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn on_checker_event(&mut self, event: CheckerEvent) -> Result<(), Error> {
        if let Status::Active { .. } = self.status.get() {
            match event {
                CheckerEvent::Progress(progress) => {
                    self.update_task_status(TaskStatus::Progress(progress))?;
                },
                CheckerEvent::Ready => {
                    self.status.update(|status| {
                        if let Status::Active { ready, .. } = status {
                            *ready = true;
                        }
                    });
                    self.update_task_status(TaskStatus::Active)?;
                },
            }
        }
        Ok(())
    }

    fn on_terminated(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::WaitContainerKilled => {
                self.status.set(Status::CleanDangling);
            },
            Status::Active { .. } => {
                // TODO: Add waiting interval + fallback
                // self.status.set(Status::CleanDangling);
            },
            _ => {},
        }
        Ok(())
    }
}
