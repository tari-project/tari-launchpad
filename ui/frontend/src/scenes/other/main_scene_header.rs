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
