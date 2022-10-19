use anyhow::Error;
use yew::{html, Html};

use crate::{
    scenes::{MainSceneHeader, WalletInfo},
    states::local_state::{LocalState, LocalStateDelta},
    widget::{Context, FromDelta, Pod, Widget},
};

pub struct MainScene {
    // local_state: Connected<LocalState>,
}

#[derive(Clone)]
pub enum Msg {
    Updated,
}

impl FromDelta<LocalState> for Msg {
    fn from_delta(_delta: &LocalStateDelta) -> Option<Self> {
        Some(Msg::Updated)
    }
}

impl Widget for MainScene {
    type Msg = Msg;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {
            // local_state: ctx.connect(&LOCAL_STATE),
        }
    }

    fn on_event(&mut self, _msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.redraw();
        Ok(())
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="main_scene">
                <div class="main_scene-content">
                    <Pod<MainSceneHeader> />
                    <Pod<WalletInfo> />
                </div>
            </div>
        })
    }
}
