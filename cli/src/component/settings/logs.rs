use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    component::{
        elements::block_with_title,
        widgets::{LabeledInput, Separator},
        Component,
        ComponentEvent,
        Frame,
        Input,
        Pass,
    },
    focus_id,
    state::{
        focus::{self, Focus},
        AppState,
    },
};

pub static LOGS_SETTINGS: Focus = focus_id!();
static MAX_SIZE: Focus = focus_id!();

pub struct LogsSettings {
    expert_sep: Separator,
    max_size: LabeledInput,
}

impl LogsSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            max_size: LabeledInput::new("Max logs file size", MAX_SIZE),
        }
    }
}

impl Input for LogsSettings {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == LOGS_SETTINGS {
            state.focus_on(MAX_SIZE);
        } else if state.focus_on == MAX_SIZE {
            let released = self.max_size.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                _ => {
                    self.max_size.on_event(event, state);
                },
            }
        } else {
            //
        }
    }
}

impl<B: Backend> Component<B> for LogsSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Logs Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [
            // Expert
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.max_size.draw(f, chunks[2], state);
    }
}
