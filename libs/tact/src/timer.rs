use std::time::Duration;

use tokio::time;

use crate::{recipient::Notifier, task::Task};

#[derive(Debug)]
pub struct Timeout {
    _task: Task,
}

impl Timeout {
    pub fn spawn<M>(duration: Duration, notifier: Notifier<M>) -> Self
    where M: Clone + Send + 'static {
        let task = Task::spawn(async move {
            time::sleep(duration).await;
            if let Err(err) = notifier.notify() {
                log::error!("Can't notify a recipient about: {err}");
            }
        });
        Self { _task: task }
    }
}

#[derive(Debug)]
pub struct Interval {
    _task: Task,
}

impl Interval {
    pub fn spawn<M>(duration: Duration, notifier: Notifier<M>) -> Self
    where M: Clone + Send + 'static {
        let task = Task::spawn(async move {
            loop {
                time::sleep(duration).await;
                if let Err(err) = notifier.notify() {
                    log::error!("Can't notify a recipient: {err}");
                }
            }
        });
        Self { _task: task }
    }
}
