use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use crate::{
    states::{
        local_state::{LocalState, LocalStateDelta, ViewMode, LOCAL_STATE},
        remote_state::RemoteState,
    },
    widget::{Connected, Context, FromDelta, Widget},
};

pub struct MainView {}

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

impl Widget for MainView {
    type Msg = Msg;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        match msg {
            Msg::Redraw => {},
        }
        ctx.redraw();
        Ok(())
    }

    fn view_opt(&self, ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="main_view" data-tauri-drag-region="">{ "main-view" }</div>
        })
    }
}
