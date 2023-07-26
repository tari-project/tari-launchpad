use futures::{Stream, StreamExt};

use crate::{recipient::Recipient, task::Task};

#[derive(Debug)]
pub struct Receiver {
    _task: Task,
}

impl Receiver {
    pub fn connect<M, S>(stream: S, recipient: Recipient<M>) -> Self
    where
        M: Send + 'static,
        S: Stream<Item = M> + Send + 'static,
    {
        let task = Task::spawn(async move {
            tokio::pin!(stream);
            while let Some(event) = stream.next().await {
                if let Err(err) = recipient.send(event) {
                    log::error!("Can't forward an item from the stream: {err}");
                }
            }
        });
        Self { _task: task }
    }
}
