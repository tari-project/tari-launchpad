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

use std::any::type_name;

use yew::{Component, Context as YewContext, Html};

use super::{base::Widget, context::Context};

pub struct Pod<W: Widget> {
    widget: W,
    context: Context<W>,
}

pub enum Msg<W: Widget> {
    WidgetMsg(W::Msg),
}

impl<W: Widget> Component for Pod<W> {
    type Message = Msg<W>;
    type Properties = ();

    fn create(ctx: &YewContext<Self>) -> Self {
        let mut context = Context::new(ctx);
        let widget = W::create(&mut context);
        Self { widget, context }
    }

    fn rendered(&mut self, _ctx: &YewContext<Self>, first_render: bool) {
        if first_render {
            if let Err(err) = self.widget.initialize(&mut self.context) {
                log::error!("Initialization failed: {}", err);
            }
        }
    }

    fn update(&mut self, _ctx: &YewContext<Self>, msg: Self::Message) -> bool {
        self.context.no_redraw();
        let res = match msg {
            Msg::WidgetMsg(msg) => self.widget.on_event(msg, &mut self.context),
        };
        if let Err(err) = res {
            log::error!("Update of {} failed: {}", type_name::<W>(), err);
        }
        self.context.should_redraw()
    }

    fn view(&self, _ctx: &YewContext<Self>) -> Html {
        self.widget.view(&self.context)
    }
}
