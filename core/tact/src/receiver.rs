use futures::{Stream, StreamExt};

use crate::{recipient::Recipient, task::Task};

#[derive(Debug)]
pub struct Receiver {
    task: Task,
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
                recipient.send(event);
            }
        });
        Self { task }
    }
}
