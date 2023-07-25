mod docker;

use crate::network::NetworkTask;
use anyhow::Error;
use derive_more::{Deref, DerefMut};
use tact::ActorContext;

#[derive(Deref, DerefMut)]
pub(super) struct NetworkTaskFsm<'a> {
    #[deref]
    #[deref_mut]
    task: &'a mut NetworkTask,
    ctx: &'a mut ActorContext<NetworkTask>,
}

impl<'a> NetworkTaskFsm<'a> {
    pub fn new(task: &'a mut NetworkTask, ctx: &'a mut ActorContext<NetworkTask>) -> Self {
        Self { task, ctx }
    }
}
