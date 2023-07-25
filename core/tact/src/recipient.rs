use crate::{
    action::Do,
    address::{Address, SendError},
};

pub trait Sender<M>: Send {
    fn send(&self, msg: M) -> Result<(), SendError>;
}

impl<A, M> Sender<M> for Address<A>
where
    A: Do<M>,
    M: Send + 'static,
{
    fn send(&self, msg: M) -> Result<(), SendError> {
        Address::send(self, msg)
    }
}

pub struct Recipient<M> {
    sender: Box<dyn Sender<M>>,
}

impl<A, M> From<Address<A>> for Recipient<M>
where
    A: Do<M>,
    M: Send + 'static,
{
    fn from(address: Address<A>) -> Self {
        Self {
            sender: Box::new(address),
        }
    }
}

impl<M> Recipient<M> {
    pub fn send(&self, msg: M) -> Result<(), SendError> {
        self.sender.send(msg)
    }
}

pub struct Notifier<M> {
    message: M,
    sender: Box<dyn Sender<M>>,
}

impl<A, M> From<(Address<A>, M)> for Notifier<M>
where
    A: Do<M>,
    M: Send + 'static,
{
    fn from((address, message): (Address<A>, M)) -> Self {
        Self {
            message,
            sender: Box::new(address),
        }
    }
}

impl<M: Clone> Notifier<M> {
    pub fn notify(&self) -> Result<(), SendError> {
        let msg = self.message.clone();
        self.sender.send(msg)
    }
}
