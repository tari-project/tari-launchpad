use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::{focus, AppState},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Expert,
    Settings,
}

/// A selector to switch between `Normal`, `Expert`, and `Settings`.
pub struct ModeSelector {
    expert: bool,
    settings: bool,
}

impl ModeSelector {
    pub fn new() -> Self {
        Self {
            expert: false,
            settings: false,
        }
    }

    pub fn selected(&self) -> Mode {
        match (self.expert, self.settings) {
            (_, true) => Mode::Settings,
            (true, false) => Mode::Expert,
            (false, false) => Mode::Normal,
        }
    }
}

impl Input for ModeSelector {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        let mut changed = false;
        if let ComponentEvent::KeyEvent(key) = event {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('n') => {
                        self.expert = false;
                        self.settings = false;
                        changed = true;
                    },
                    KeyCode::Char('e') => {
                        self.expert = !self.expert;
                        self.settings = false;
                        changed = true;
                    },
                    KeyCode::Char('s') => {
                        self.settings = !self.settings;
                        changed = true;
                    },
                    _ => {},
                }
            }
        }
        if changed {
            state.focus_on(focus::ROOT);
        }
    }
}

impl<B: Backend> Component<B> for ModeSelector {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let not_selected = Style::default().fg(Color::White);
        let selected = Style::default().fg(Color::Magenta);
        let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
        let style_for = |mode: Mode| -> Style {
            if mode == self.selected() {
                selected
            } else {
                not_selected
            }
        };
        let selector = if self.expert { " o" } else { "o " };
        let spans = Spans(vec![
            Span::styled("Normal", style_for(Mode::Normal)),
            Span::raw(" ("),
            Span::styled(selector, bold),
            Span::raw(") "),
            Span::styled("Expert", style_for(Mode::Expert)),
            Span::raw(" | "),
            Span::styled("Settings", style_for(Mode::Settings)),
        ]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Right);
        f.render_widget(paragraph, rect);
    }
}
