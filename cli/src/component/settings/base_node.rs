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

pub static BASE_NODE_SETTINGS: Focus = focus_id!();
static ROOT_FOLDER: Focus = focus_id!();

pub struct BaseNodeSettings {
    expert_sep: Separator,
    root_folder: LabeledInput,
}

impl BaseNodeSettings {
    pub fn new() -> Self {
        Self {
            expert_sep: Separator::new("Expert", []),
            root_folder: LabeledInput::new("Root folder", ROOT_FOLDER),
        }
    }
}

impl Input for BaseNodeSettings {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == BASE_NODE_SETTINGS {
            state.focus_on(ROOT_FOLDER);
        } else if state.focus_on == ROOT_FOLDER {
            let released = self.root_folder.is_released();
            match event.pass() {
                Pass::Up | Pass::Leave if released => {
                    state.focus_on(focus::ROOT);
                },
                _ => {
                    self.root_folder.on_event(event, state);
                },
            }
        } else {
            //
        }
    }
}

impl<B: Backend> Component<B> for BaseNodeSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("BaseNode Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [Constraint::Length(1), Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.expert_sep.draw(f, chunks[0], state);
        self.root_folder.draw(f, chunks[1], state);
    }
}
