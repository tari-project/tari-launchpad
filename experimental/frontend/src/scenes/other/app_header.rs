use anyhow::Error;
use strum::IntoEnumIterator;
use yew::{classes, html, Html};

use crate::{
    states::local_state::{LocalState, LocalStateDelta, ViewMode, LOCAL_STATE},
    widget::{Connected, Context, FromDelta, Widget},
};

pub struct AppHeader {
    local_state: Connected<LocalState>,
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

impl Widget for AppHeader {
    type Msg = Msg;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            local_state: ctx.connect(&LOCAL_STATE),
        }
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
            <div class="menu">
                { for ViewMode::iter().map(|item| self.render_item(item, ctx)) }
            </div>
        })
    }
}

impl AppHeader {
    fn render_item(&self, view_mode: ViewMode, _ctx: &Context<Self>) -> Html {
        let event = LocalStateDelta::SetViewMode(view_mode.clone());
        let onclick = self.local_state.event(event);
        let selected = (self.local_state.get().view_mode == view_mode).then(|| "selected");
        html! {
            <div class={classes!("menu_item", selected)} {onclick}>{ view_mode }</div>
        }
    }
}
