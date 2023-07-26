use futures::Future;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct Task {
    handle: JoinHandle<()>,
}

impl Drop for Task {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl Task {
    pub fn spawn<T>(fut: T) -> Self
    where T: Future<Output = ()> + Send + 'static {
        let handle = tokio::spawn(fut);
        Self { handle }
    }
}
