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

use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use crate::{
    states::{
        local_state::{LocalState, LocalStateDelta, LOCAL_STATE},
        remote_state::{RemoteState, REMOTE_STATE},
    },
    widget::{Connected, Context, FromDelta, Widget},
};

pub struct LogsView {
    local_state: Connected<LocalState>,
    remote_state: Connected<RemoteState>,
}

#[derive(Clone)]
pub enum Msg {
    Redraw,
}

impl FromDelta<LocalState> for Msg {
    fn from_delta(_delta: &LocalStateDelta) -> Option<Self> {
        Some(Self::Redraw)
    }
}

impl FromDelta<RemoteState> for Msg {
    fn from_delta(_delta: &Reaction) -> Option<Self> {
        Some(Self::Redraw)
    }
}

impl Widget for LogsView {
    type Msg = Msg;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            local_state: ctx.connect(&LOCAL_STATE),
            remote_state: ctx.connect(&REMOTE_STATE),
        }
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        match msg {
            Msg::Redraw => {},
        }
        ctx.redraw();
        Ok(())
    }

    fn view_empty(&self) -> Html {
        html! {
            <div>{ "Choose a container to open logs" }</div>
        }
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html> {
        let local_state = self.local_state.get();
        let remote_state = self.remote_state.get();
        let logs_id = local_state.show_logs_for.as_ref()?;
        let record = remote_state.state.containers.get(logs_id)?;
        let tail = &record.tail;
        Some(html! {
            <div class="table">
                <div>{ format!("Logs of {}", logs_id) }</div>
                { for tail.iter().take(5).map(|line| self.render_line(line)) }
            </div>
        })
    }
}

impl LogsView {
    fn render_line(&self, line: &str) -> Html {
        html! {
            <div>{ line }</div>
        }
    }
}
