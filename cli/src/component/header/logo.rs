use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    component::{Component, Frame},
    state::AppState,
};

pub struct Logo {}

impl Logo {
    pub fn new() -> Self {
        Self {}
    }
}

impl<B: Backend> Component<B> for Logo {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
        let line = Line::from(vec![
            Span::styled("Tari", bold),
            Span::raw(" "),
            Span::styled("Launchpad", bold),
            Span::raw(" "),
            Span::styled("App", bold),
        ]);
        let text = vec![line];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
