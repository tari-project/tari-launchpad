mod docker;
mod events;
mod update;

use crate::container::{ContainerTask, ProcessChanges, Status};
use crate::types::TaskStatus;
use anyhow::Error;
use derive_more::{Deref, DerefMut};
use tact::ActorContext;

#[derive(Deref, DerefMut)]
pub(super) struct ContainerTaskFsm<'a> {
    #[deref]
    #[deref_mut]
    task: &'a mut ContainerTask,
    ctx: &'a mut ActorContext<ContainerTask>,
}

impl<'a> ContainerTaskFsm<'a> {
    pub fn new(task: &'a mut ContainerTask, ctx: &'a mut ActorContext<ContainerTask>) -> Self {
        Self { task, ctx }
    }

    fn get_status(&self) -> &Status {
        &self.task.status
    }

    fn set_status(&mut self, status: Status) -> Result<(), Error> {
        self.task.status = status;
        self.ctx.do_next(ProcessChanges)?;
        Ok(())
    }

    fn update_task_status(&mut self, task_status: TaskStatus) -> Result<(), Error> {
        self.task.task_status = task_status;
        log::error!("TODO: Forward the task status to the bus");
        Ok(())
    }
}
