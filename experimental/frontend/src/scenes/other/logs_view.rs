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
