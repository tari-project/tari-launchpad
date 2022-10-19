use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use crate::{
    states::{
        local_state::{LocalState, LocalStateDelta},
        remote_state::{RemoteState, REMOTE_STATE},
    },
    widget::{Connected, Context, FromDelta, Widget},
};

pub struct WalletInfo {
    // local_state: Connected<LocalState>,
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

impl Widget for WalletInfo {
    type Msg = Msg;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            // local_state: ctx.connect(&LOCAL_STATE),
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
        let remote_state = self.remote_state.get();
        let wallet = &remote_state.state.wallet;
        Some(html! {
            <div>
            { format!("{:?}", wallet) }
            </div>
        })
    }
}
