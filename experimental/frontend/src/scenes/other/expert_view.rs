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
