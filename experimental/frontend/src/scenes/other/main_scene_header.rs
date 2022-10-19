use anyhow::Error;
use strum::IntoEnumIterator;
use yew::{classes, html, Html};

use crate::{
    states::local_state::{LocalState, LocalStateDelta, Scene, LOCAL_STATE},
    widget::{AcceptAll, Connected, Context, Widget},
};

pub struct MainSceneHeader {
    local_state: Connected<LocalState>,
}

impl Widget for MainSceneHeader {
    type Msg = AcceptAll;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            local_state: ctx.connect(&LOCAL_STATE),
        }
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        log::info!("Event received: {:?}", msg);
        ctx.redraw();
        Ok(())
    }

    fn view_opt(&self, ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="menu">
                { for Scene::iter().map(|item| self.render_item(item, ctx)) }
            </div>
        })
    }
}

impl MainSceneHeader {
    fn render_item(&self, scene: Scene, _ctx: &Context<Self>) -> Html {
        let event = LocalStateDelta::SetScene(scene.clone());
        let onclick = self.local_state.event(event);
        let selected = (self.local_state.get().scene == scene).then(|| "selected");
        html! {
            <div class={classes!("menu_item", selected)} {onclick}>{ scene }</div>
        }
    }
}
