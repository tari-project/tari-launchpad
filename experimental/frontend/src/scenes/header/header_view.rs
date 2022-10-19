use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use super::{ControlButtons, TopMenu};
use crate::{
    scenes::{icons, MainView},
    states::{
        local_state::{LocalState, LocalStateDelta, ViewMode, LOCAL_STATE},
        remote_state::RemoteState,
    },
    widget::{Connected, Context, FromDelta, Pod, Widget},
};

pub struct HeaderView {}

#[derive(Clone)]
pub enum Msg {}

impl FromDelta<LocalState> for Msg {}

impl FromDelta<RemoteState> for Msg {}

impl Widget for HeaderView {
    type Msg = Msg;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn view_opt(&self, ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div data-tauri-drag-region="" class="header_view">
                <Pod<ControlButtons> />
                <div class="logo">{ icons::logo::render() }</div>
                <div class="separator" />
                <Pod<TopMenu> />
            </div>
        })
    }
}
