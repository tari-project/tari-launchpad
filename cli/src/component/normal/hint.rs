use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct HintLine<T> {
    getter: T,
}

pub trait HintGetter {
    fn get_hint(&self, state: &AppState) -> String;
}

impl<T> HintLine<T> {
    pub fn new(getter: T) -> Self {
        Self { getter }
    }
}

impl<T> Input for HintLine<T> {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend, T> Component<B> for HintLine<T>
where T: HintGetter
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let text = self.getter.get_hint(state);
        let spans = Spans(vec![Span::raw(text)]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
