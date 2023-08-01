use thiserror::Error;
use tokio::sync::{mpsc, watch};

use crate::{
    action::{Do, Interrupt},
    actor::Actor,
    handler::Envelope,
    joint::ActorState,
};

#[derive(Debug, Error)]
#[error("Can't send an event to an actor")]
pub struct SendError;

pub struct Address<A: Actor> {
    tx_event: mpsc::UnboundedSender<Envelope<A>>,
    rx_state: watch::Receiver<ActorState>,
}

impl<A: Actor> Clone for Address<A> {
    fn clone(&self) -> Self {
        Self {
            tx_event: self.tx_event.clone(),
            rx_state: self.rx_state.clone(),
        }
    }
}

impl<A: Actor> Address<A> {
    pub(super) fn new(tx_event: mpsc::UnboundedSender<Envelope<A>>, rx_state: watch::Receiver<ActorState>) -> Self {
        Self { tx_event, rx_state }
    }

    pub fn send<E>(&self, event: E) -> Result<(), SendError>
    where
        A: Do<E>,
        E: Send + 'static,
    {
        let envelope = Envelope::from_event(event);
        self.tx_event.send(envelope).map_err(|_| SendError)
    }

    pub fn interrupt(&mut self) -> Result<(), SendError>
    where A: Do<Interrupt> {
        self.send(Interrupt)
    }

    pub async fn join(&mut self) -> Result<(), SendError> {
        loop {
            let state = self.rx_state.borrow_and_update().clone();
            if state == ActorState::Finished {
                break;
            }
            self.rx_state.changed().await.map_err(|_| SendError)?;
        }
        Ok(())
    }
}
