use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

pub fn block_with_title(title: Option<&str>, focus: bool) -> Block<'_> {
    let color = if focus { Color::Magenta } else { Color::White };
    let block = Block::default()
        .border_style(Style::default().fg(color))
        .borders(Borders::ALL);
    if let Some(title) = title {
        let title = format!(" {title} ");
        let style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
        let title_span = Span::styled(title, style);
        block.title(title_span)
    } else {
        block
    }
}

pub fn logo(logo: &str) -> Paragraph<'_> {
    let text = logo.trim_start_matches(char::is_whitespace);
    Paragraph::new(text)
}
