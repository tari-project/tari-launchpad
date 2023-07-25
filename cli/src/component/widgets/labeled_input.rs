use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tui_textarea::TextArea;

use crate::{
    component::{widgets::Label, Component, Frame},
    state::AppState,
};

pub struct LabeledInput {
    label: String,
    input: TextArea<'static>,
}

impl LabeledInput {
    pub fn new(label: impl ToString) -> Self {
        let input = TextArea::default();
        let mut this = Self {
            label: label.to_string(),
            input,
        };
        this.set_focus(false);
        this
    }

    fn set_focus(&mut self, focus: bool) {
        let (block_color, cursor_color) = {
            if focus {
                (Color::Magenta, Color::White)
            } else {
                (Color::White, Color::Reset)
            }
        };
        let block = Block::default()
            .border_style(Style::default().fg(block_color))
            .borders(Borders::ALL);
        self.input.set_block(block);
        self.input.set_cursor_style(Style::default().bg(cursor_color));
    }
}

impl<B: Backend> Component<B> for LabeledInput {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let constraints = [Constraint::Percentage(40), Constraint::Percentage(60)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        let label = Label::new(&self.label);
        f.render_widget(label, h_chunks[0]);
        let input = self.input.widget();
        f.render_widget(input, h_chunks[1]);
    }
}
