use std::collections::HashMap;

use tui::{backend::Backend, layout::Rect, terminal::Frame};

use crate::{
    component::{Component, ComponentEvent, Input, Pass},
    state::{AppState, Focus},
};

pub trait Focusable {
    fn focus(&mut self, focus: bool);
}

pub struct Motion<T> {
    focus: Focus,
    inner: T,
    directions: HashMap<Pass, Focus>,
}

impl<T> Input for Motion<T>
where T: Focusable + Input
{
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        let pass = event.pass();
        if let Some(focus) = self.directions.get(&pass) {
            state.focus_on(focus.clone());
        } else {
            self.inner.on_event(event, state);
        }
        self.inner.focus(self.focus == state.focus_on);
    }
}

impl<B: Backend, T> Component<B> for Motion<T>
where T: Component<B, State = AppState>
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        self.inner.draw(f, rect, state);
    }
}
