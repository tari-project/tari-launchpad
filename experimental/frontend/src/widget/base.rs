use anyhow::Error;
use yew::Html;

use super::{context::Context, subscribe::FromDelta};
use crate::states::{local_state::LocalState, remote_state::RemoteState};

pub trait Widget: Sized + 'static {
    type Msg: FromDelta<LocalState> + FromDelta<RemoteState> + Send;

    fn create(ctx: &mut Context<Self>) -> Self;

    fn initialize(&mut self, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }

    fn on_event(&mut self, _msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.redraw();
        Ok(())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.view_opt(ctx).unwrap_or_else(|| self.view_empty())
    }

    fn view_empty(&self) -> Html {
        Html::default()
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html>;
}
