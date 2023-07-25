use std::time::Duration;

use tokio::time;

use crate::{recipient::Notifier, task::Task};

#[derive(Debug)]
pub struct Timeout {
    task: Task,
}

impl Timeout {
    pub fn spawn<M>(duration: Duration, notifier: Notifier<M>) -> Self
    where M: Clone + Send + 'static {
        let task = Task::spawn(async move {
            time::sleep(duration.into()).await;
            notifier.notify();
        });
        Self { task }
    }
}

#[derive(Debug)]
pub struct Interval {
    task: Task,
}

impl Interval {
    pub fn spawn<M>(duration: Duration, notifier: Notifier<M>) -> Self
    where M: Clone + Send + 'static {
        let task = Task::spawn(async move {
            loop {
                time::sleep(duration.into()).await;
                notifier.notify();
            }
        });
        Self { task }
    }
}
