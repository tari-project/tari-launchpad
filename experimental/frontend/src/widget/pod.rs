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
