use crate::container::{CheckerEvent, ContainerTaskFsm, Event, Status};
use crate::types::{TaskProgress, TaskStatus};
use anyhow::Error;

impl<'a> ContainerTaskFsm<'a> {
    pub fn process_event(&mut self, event: Event) -> Result<(), Error> {
        log::warn!("EVENT: {event:?}");
        match event {
            Event::Created => self.on_created(),
            Event::PullingProgress(value) => self.on_pulling_progress(value),
            Event::Destroyed => self.on_destroyed(),
            Event::Started => self.on_started(),
            Event::Killed => self.on_killed(),
            Event::Terminated => self.on_terminated(),
            Event::CheckerEvent(event) => self.on_checker_event(event),
        }
    }

    fn on_created(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerCreated = self.get_status() {
            self.set_status(Status::StartContainer)?;
        }
        Ok(())
    }

    fn on_pulling_progress(&mut self, value: TaskProgress) -> Result<(), Error> {
        if let Status::PullingImage { .. } = self.get_status() {
            self.update_task_status(TaskStatus::Progress(value))?;
        }
        Ok(())
    }

    fn on_destroyed(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerRemoved = self.get_status() {
            self.set_status(Status::CleanDangling)?;
        }
        Ok(())
    }

    fn on_started(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerStarted { .. } = self.get_status() {
            /*
            let checker = self.inner.image.checker();
            let logs = self.logs_stream();
            let stats = self.stats_stream();
            let sender = self.sender().clone();
            let context = CheckerContext::new(logs, stats, sender);
            let fur = checker.entrypoint(context);
            let checker = tokio::spawn(fur).into();
            self.status.set(Status::Active { checker, ready: false });
            */
        }
        Ok(())
    }

    fn on_killed(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn on_checker_event(&mut self, event: CheckerEvent) -> Result<(), Error> {
        if let Status::Active { .. } = self.get_status() {
            match event {
                CheckerEvent::Progress(progress) => {
                    self.update_task_status(TaskStatus::Progress(progress))?;
                }
                CheckerEvent::Ready => {
                    if let Status::Active { ready, .. } = &mut self.task.status {
                        *ready = true;
                    }
                    self.update_task_status(TaskStatus::Active)?;
                }
            }
        }
        Ok(())
    }

    fn on_terminated(&mut self) -> Result<(), Error> {
        match self.get_status() {
            Status::WaitContainerKilled => {
                self.set_status(Status::CleanDangling)?;
            }
            Status::Active { .. } => {
                // TODO: Add waiting interval + fallback
                // self.set_status(Status::CleanDangling)?;
            }
            _ => {}
        }
        Ok(())
    }
}
