use std::collections::HashSet;

use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::line,
    text::Span,
    widgets::Widget,
    Frame,
};

use crate::{
    component::{AppState, Component},
    state::Focus,
};

pub struct Separator {
    focus_on: HashSet<Focus>,
    title: String,
}

impl Separator {
    pub fn new(title: &str, focus: impl IntoIterator<Item = Focus>) -> Self {
        Self {
            focus_on: focus.into_iter().collect(),
            title: title.into(),
        }
    }
}

impl<B: Backend> Component<B> for Separator {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let render = Render {
            focus: self.focus_on.contains(&state.focus_on),
            title: &self.title,
            line_set: line::NORMAL,
        };
        f.render_widget(render, rect);
    }
}

struct Render<'a> {
    focus: bool,
    title: &'a str,
    line_set: line::Set,
}

impl<'a> Widget for Render<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color = if self.focus { Color::Magenta } else { Color::White };
        let modifier = if self.focus { Modifier::BOLD } else { Modifier::empty() };
        let style = Style::default().fg(color).add_modifier(modifier);
        let span = Span::styled(self.title, style);
        let (col, row) = buf.set_span(area.left(), area.top(), &span, area.width);
        let start = col + 1;

        let y = row;
        for x in start..area.right() {
            buf.get_mut(x, y).set_symbol(self.line_set.horizontal); //"_"
        }
    }
}
