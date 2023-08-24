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
use tari_launchpad_protocol::{
    container::{TaskId, TaskState},
    launchpad::{Action, Reaction},
};
use yew::{classes, html, Html};

use super::LogsView;
use crate::{
    bus,
    states::{
        local_state::{LocalState, LocalStateDelta, LOCAL_STATE},
        remote_state::{RemoteState, REMOTE_STATE},
    },
    widget::{Connected, Context, FromDelta, Pod, Widget},
};

pub struct ExpertView {
    local_state: Connected<LocalState>,
    remote_state: Connected<RemoteState>,
}

#[derive(Clone)]
pub enum Msg {
    Redraw,
    Start,
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

impl Widget for ExpertView {
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
            Msg::Start => {
                log::info!("Starting...");
                bus::request(Action::Start);
            },
        }
        ctx.redraw();
        Ok(())
    }

    fn view_opt(&self, ctx: &Context<Self>) -> Option<Html> {
        let remote_state = self.remote_state.get();
        let config = &remote_state.state;
        let selected = config.active.then(|| "selected");
        let class = classes!("menu_item", selected);
        Some(html! {
            <div>
                <div>{ "Expert View" }</div>
                <div class="menu">
                    <div { class } onclick={ctx.event(Msg::Start)} >{ "Start" }</div>
                </div>
                // { format!("{:?}", *remote_state) }
                { self.render_table(ctx) }
                <Pod<LogsView> />
            </div>
        })
    }
}

impl ExpertView {
    fn render_table(&self, ctx: &Context<Self>) -> Html {
        let remote_state = self.remote_state.get();
        html! {
            <div class="table">
                { for remote_state.state.containers.iter().map(|(k, v)| self.render_row(k, v, ctx)) }
            </div>
        }
    }

    fn render_row(&self, task_id: &TaskId, state: &TaskState, _ctx: &Context<Self>) -> Html {
        let event = LocalStateDelta::ShowLogs(task_id.clone());
        let onclick = self.local_state.event(event);
        // let selected = (self.local_state.get().view_mode == view_mode).then(|| "selected");
        html! {
            <div class="row" {onclick}>
                <div>{ task_id }</div>
                <div>{ &state.status }</div>
            </div>
        }
    }
}
