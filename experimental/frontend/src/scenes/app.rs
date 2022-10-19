use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use crate::{
    scenes::{HeaderView, MainView},
    states::{
        local_state::{LocalState, LocalStateDelta, ViewMode, LOCAL_STATE},
        remote_state::RemoteState,
    },
    widget::{Connected, Context, FromDelta, Pod, Widget},
};

pub struct App {}

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

impl Widget for App {
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
            <div class="app_view" data-tauri-drag-region="">
                // TODO: Render a global header
                <Pod<HeaderView> />
                <Pod<MainView> />
                // TODO: Render `ExpertView`
                // TODO: Render `Settings`
                // TODO: Render `Modal`
            </div>
        })
    }
}
