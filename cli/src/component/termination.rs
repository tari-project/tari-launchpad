use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::component::{elements::block_with_title, AppState, Component};

pub struct TerminationView {}

impl TerminationView {
    pub fn new() -> Self {
        Self {}
    }
}

impl<B: Backend> Component<B> for TerminationView {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(None, false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [Constraint::Percentage(40), Constraint::Length(1)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        let paragraph = Paragraph::new("Terminating...").alignment(Alignment::Center);
        f.render_widget(paragraph, v_chunks[1]);
    }
}
