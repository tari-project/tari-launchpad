use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub trait StatusGetter {
    fn get_status(&self, state: &AppState) -> (&str, Color);
}

pub struct StatusBadge<G> {
    getter: G,
}

impl<G> StatusBadge<G> {
    pub fn new(getter: G) -> Self {
        Self { getter }
    }
}

impl<G> Input for StatusBadge<G> {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend, G> Component<B> for StatusBadge<G>
where G: StatusGetter
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let (text, color) = self.getter.get_status(state);
        let style = Style::default().fg(color);
        let spans = Spans(vec![Span::styled(text, style)]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
