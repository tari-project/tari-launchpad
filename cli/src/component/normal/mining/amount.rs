use rust_decimal::Decimal;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub trait AmountGetter {
    fn get_amount(&self, state: &AppState) -> (Decimal, &str);
}

pub struct AmountIndicator<G> {
    getter: G,
}

impl<G> AmountIndicator<G> {
    pub fn new(getter: G) -> Self {
        Self { getter }
    }
}

impl<G> Input for AmountIndicator<G> {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend, G> Component<B> for AmountIndicator<G>
where G: AmountGetter
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let (amount, curr) = self.getter.get_amount(state);
        let s = amount.to_string();

        let spans = Spans(vec![
            Span::raw(s),
            Span::raw(" "),
            Span::styled(curr, Style::default().add_modifier(Modifier::BOLD)),
        ]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
