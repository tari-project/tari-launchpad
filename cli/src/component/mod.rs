mod elements;
mod expert;
mod header;
mod main_view;
mod motion;
mod normal;
mod onboarding;
mod scene;
mod settings;
mod tabs;
mod widgets;

use crossterm::event::{KeyCode, KeyEvent};
use derive_more::From;
pub use main_view::MainView;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::state::AppState;

pub trait Component<B: Backend> {
    type State;

    /// A context reference a mutable to modify the frame.
    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pass {
    Up,
    Down,
    Left,
    Right,
    // Esc
    Leave,
    Enter,
    Space,
    // Tab
    Next,
    Other,
    Tick,
}

impl Pass {
    fn any(&self, arr: &[Pass]) -> bool {
        for item in arr {
            if item == self {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, From)]
pub enum ComponentEvent {
    KeyEvent(KeyEvent),
    Tick,
}

impl ComponentEvent {
    pub fn pass(&self) -> Pass {
        match self {
            Self::KeyEvent(event) => match event.code {
                KeyCode::Up | KeyCode::Char('k') => Pass::Up,
                KeyCode::Down | KeyCode::Char('j') => Pass::Down,
                KeyCode::Left | KeyCode::Char('h') => Pass::Left,
                KeyCode::Right | KeyCode::Char('l') => Pass::Right,
                KeyCode::Esc => Pass::Leave,
                KeyCode::Enter => Pass::Enter,
                KeyCode::Char(' ') => Pass::Space,
                KeyCode::Tab => Pass::Next,
                _ => Pass::Other,
            },
            Self::Tick => Pass::Tick,
        }
    }
}

pub trait Input {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState);
}
