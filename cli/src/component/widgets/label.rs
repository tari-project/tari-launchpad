use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::Widget,
};

pub struct Label<'a> {
    text: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Widget for Label<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().add_modifier(Modifier::BOLD);
        let span = Span::styled(self.text, style);
        let top = area.top() + area.height / 2;
        let left = area.left();
        let width = area.width;
        buf.set_span(left, top, &span, width);
    }
}
