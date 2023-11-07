// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

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
