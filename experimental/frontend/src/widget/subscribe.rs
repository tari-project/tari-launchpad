use std::{
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use derive_more::{Deref, From, Into};
use futures::{channel::mpsc, StreamExt};
use once_cell::sync::Lazy;
use slab::Slab;
use yew::Callback;

use crate::widget::{context::PodScope, pod::Msg, Widget};

#[derive(Deref)]
pub struct SharedState<T: State> {
    lazy_state: Lazy<SharedStateExtern<T>>,
}

impl<T: State> SharedState<T> {
    pub const fn new() -> Self {
        Self {
            lazy_state: Lazy::new(SharedStateExtern::new),
        }
    }
}

#[derive(Deref)]
pub struct Connected<T: State> {
    #[deref]
    shared_state: SharedStateExtern<T>,
    idx: CallbackIdx,
}

impl<T: State> Drop for Connected<T> {
    fn drop(&mut self) {
        self.shared_state.unregister(self.idx);
    }
}

pub struct ConnectedState<'a, T: State> {
    guard: RwLockReadGuard<'a, SharedStateInner<T>>,
}

impl<'a, T: State> Deref for ConnectedState<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard.state
    }
}

impl<T: State> Connected<T> {
    pub fn get(&self) -> ConnectedState<'_, T> {
        let guard = self.shared_state.inner.read().unwrap();
        ConnectedState { guard }
    }
}

#[derive(Debug, Clone)]
pub struct AcceptAll;

impl<T: State> FromDelta<T> for AcceptAll {
    fn from_delta(_delta: &T::Delta) -> Option<Self> {
        Some(Self)
    }
}

#[derive(Debug, Clone)]
pub struct IgnoreAll;

impl<T: State> FromDelta<T> for IgnoreAll {
    fn from_delta(_delta: &T::Delta) -> Option<Self> {
        None
    }
}

#[derive(From, Into, Clone, Copy)]
struct CallbackIdx(usize);

trait Listener<T: State>: Send + Sync {
    fn send(&self, delta: &T::Delta);
}

impl<T: State, M> Listener<T> for mpsc::UnboundedSender<M>
where M: FromDelta<T> + Send
{
    fn send(&self, delta: &T::Delta) {
        if let Some(event) = M::from_delta(delta) {
            if let Err(err) = self.unbounded_send(event) {
                log::error!("Can't send a delta: {}", err);
            }
        }
    }
}

pub trait FromDelta<T: State>: Sized {
    fn from_delta(_delta: &T::Delta) -> Option<Self> {
        None
    }
}

pub trait State: Default + 'static {
    type Delta: Clone;

    fn apply(&mut self, delta: Self::Delta);
}

pub struct SharedStateExtern<T: State> {
    inner: Arc<RwLock<SharedStateInner<T>>>,
    sender: mpsc::UnboundedSender<T::Delta>,
}

impl<T: State> Clone for SharedStateExtern<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl<T: State> SharedStateExtern<T> {
    fn new() -> Self {
        let state = T::default();
        let inner = SharedStateInner {
            state,
            listeners: Slab::new(),
        };
        let inner = Arc::new(RwLock::new(inner));
        let sender = Worker::spawn(inner.clone());
        Self { inner, sender }
    }

    pub fn update(&self, delta: T::Delta) {
        if let Err(err) = self.sender.unbounded_send(delta) {
            log::error!("Can't send update to the bus '{}': {}", std::any::type_name::<T>(), err);
        }
    }

    /// Creates a callback to send an event to the bus.
    pub fn event<IN>(&self, delta: T::Delta) -> Callback<IN> {
        let state = self.clone();
        Callback::from(move |_: IN| {
            state.update(delta.clone());
        })
    }

    pub(super) fn register<W: Widget>(&self, ctx: &PodScope<W>) -> Connected<T>
    where W::Msg: FromDelta<T> {
        let mut inner = self.inner.write().unwrap();
        let listener = Connector::spawn_for(ctx);
        let idx = inner.listeners.insert(listener);
        Connected {
            shared_state: self.clone(),
            idx: idx.into(),
        }
    }

    fn unregister(&self, idx: CallbackIdx) {
        let mut inner = self.inner.write().unwrap();
        inner.listeners.remove(idx.into());
    }
}

pub struct SharedStateInner<T: State> {
    state: T,
    listeners: Slab<Box<dyn Listener<T>>>,
}

struct Connector<W: Widget> {
    link: PodScope<W>,
    receiver: mpsc::UnboundedReceiver<W::Msg>,
}

impl<W: Widget> Connector<W> {
    fn spawn_for<T: State>(link: &PodScope<W>) -> Box<dyn Listener<T>>
    where W::Msg: FromDelta<T> {
        let (tx, rx) = mpsc::unbounded();
        let this = Self {
            link: link.clone(),
            receiver: rx,
        };
        wasm_bindgen_futures::spawn_local(this.entrypoint());
        Box::new(tx)
    }

    async fn entrypoint(mut self) {
        while let Some(event) = self.receiver.next().await {
            let msg = Msg::WidgetMsg(event);
            self.link.send_message(msg);
        }
    }
}

struct Worker<T: State> {
    inner: Arc<RwLock<SharedStateInner<T>>>,
    receiver: mpsc::UnboundedReceiver<T::Delta>,
}

impl<T: State> Worker<T> {
    fn spawn(inner: Arc<RwLock<SharedStateInner<T>>>) -> mpsc::UnboundedSender<T::Delta> {
        let (tx, rx) = mpsc::unbounded();
        let this = Self { inner, receiver: rx };
        wasm_bindgen_futures::spawn_local(this.entrypoint());
        tx
    }

    async fn entrypoint(mut self) {
        log::info!("Worker for the state started: {}", std::any::type_name::<T>());
        while let Some(delta) = self.receiver.next().await {
            let mut inner = self.inner.write().unwrap();
            inner.state.apply(delta.clone());
            for (_idx, listener) in &inner.listeners {
                log::info!("Distribute delta to subscribers");
                listener.send(&delta);
            }
        }
    }
}
