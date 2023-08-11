use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    component::{widgets::Label, Component, ComponentEvent, Frame, Input},
    state::{AppState, Focus},
};

pub struct LabeledInput {
    input_mode: bool,
    label: String,
    content: String,
    focus: Focus,
}

impl LabeledInput {
    pub fn new(label: impl ToString, focus: Focus) -> Self {
        Self {
            input_mode: false,
            label: label.to_string(),
            content: String::new(),
            focus,
        }
    }

    pub fn is_released(&self) -> bool {
        !self.input_mode
    }
}

impl Input for LabeledInput {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == self.focus {
            if let ComponentEvent::KeyEvent(key) = event {
                if self.input_mode {
                    // TODO: Show the cursor
                    match key.code {
                        KeyCode::Char(c) => {
                            self.content.push(c);
                        },
                        KeyCode::Backspace => {
                            if key.modifiers.contains(KeyModifiers::ALT) {
                                self.content.clear();
                            } else {
                                self.content.pop();
                            }
                        },
                        KeyCode::Esc => {
                            self.input_mode = false;
                        },
                        _ => {},
                    }
                } else if let KeyCode::Enter = key.code {
                    self.input_mode = true;
                } else {
                    //
                }
            }
        }
    }
}

impl<B: Backend> Component<B> for LabeledInput {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Percentage(40), Constraint::Percentage(60)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        let label = Label::new(&self.label);
        f.render_widget(label, h_chunks[0]);

        let (block_color, _cursor_color) = {
            if state.focus_on == self.focus {
                if self.input_mode {
                    (Color::Yellow, Color::White)
                } else {
                    (Color::Magenta, Color::White)
                }
            } else {
                (Color::White, Color::Reset)
            }
        };
        let block = Block::default()
            .border_style(Style::default().fg(block_color))
            .borders(Borders::ALL);
        let s: &str = self.content.as_ref();
        let text = Paragraph::new(s).block(block);
        f.render_widget(text, h_chunks[1]);
        // let input = self.input.widget();
        // f.render_widget(input, h_chunks[1]);
    }
}
