use derive_more::{Deref, DerefMut};
use yew::{html::Scope, Callback, Context as YewContext};

use super::{
    base::Widget,
    pod::{Msg, Pod},
    subscribe::{Connected, FromDelta, SharedState, State},
};
use crate::{
    states::{
        local_state::{LocalState, LOCAL_STATE},
        remote_state::{RemoteState, REMOTE_STATE},
    },
    widget::subscribe::ConnectedState,
};

/// The scope that extends [`Scope`] to have extra
/// methods to construct special callbacks for the `Widget`.
#[derive(Deref, DerefMut)]
pub struct PodScope<W: Widget> {
    scope: Scope<Pod<W>>,
}

impl<W: Widget> Clone for PodScope<W> {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope.clone(),
        }
    }
}

impl<W: Widget> PodScope<W> {
    pub fn connect<T>(&mut self, state: &SharedState<T>) -> Connected<T>
    where
        T: State,
        W::Msg: FromDelta<T>,
    {
        state.register(self)
    }

    pub fn callback<F, IN>(&self, f: F) -> Callback<IN>
    where
        F: Fn(IN) -> W::Msg,
        F: 'static,
    {
        let f = move |input| Msg::WidgetMsg(f(input));
        self.scope.callback(f)
    }

    pub fn event<IN>(&self, msg: W::Msg) -> Callback<IN>
    where W::Msg: Clone {
        self.scope.callback(move |_: IN| Msg::WidgetMsg(msg.clone()))
    }
}

#[derive(Deref, DerefMut)]
pub struct Context<W: Widget> {
    redraw: bool,
    #[deref]
    #[deref_mut]
    pod_scope: PodScope<W>,
    local_state: Connected<LocalState>,
    remote_state: Connected<RemoteState>,
}

impl<W: Widget> Context<W> {
    pub(super) fn new(ctx: &YewContext<Pod<W>>) -> Self {
        let scope = ctx.link().clone();
        let mut pod_scope = PodScope { scope };
        let local_state = pod_scope.connect(&LOCAL_STATE);
        let remote_state = pod_scope.connect(&REMOTE_STATE);
        Self {
            redraw: false,
            pod_scope,
            local_state,
            remote_state,
        }
    }
}

impl<W: Widget> Context<W> {
    pub fn should_redraw(&self) -> bool {
        self.redraw
    }

    pub fn no_redraw(&mut self) {
        self.redraw = false;
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }

    pub fn local(&self) -> ConnectedState<'_, LocalState> {
        self.local_state.get()
    }

    pub fn remote(&self) -> ConnectedState<'_, RemoteState> {
        self.remote_state.get()
    }

    // pub fn link(&self) -> &PodScope<W> {
    // &self.pod_scope
    // }
}
