use strum::{Display, EnumIter};
use tari_launchpad_protocol::container::TaskId;

use crate::widget::{SharedState, State};

pub static LOCAL_STATE: SharedState<LocalState> = SharedState::new();

#[derive(EnumIter, Display, Clone, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Normal,
    Expert,
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(EnumIter, Display, Clone, Debug, PartialEq, Eq)]
pub enum Scene {
    Mining,
    #[strum(to_string = "Base Node")]
    BaseNode,
    Wallet,
}

impl Default for Scene {
    fn default() -> Self {
        Self::BaseNode
    }
}

#[derive(Default, Debug)]
pub struct LocalState {
    pub scene: Scene,
    pub view_mode: ViewMode,
    pub show_logs_for: Option<TaskId>,
    pub expert_view: bool,
}

impl State for LocalState {
    type Delta = LocalStateDelta;

    fn apply(&mut self, delta: Self::Delta) {
        match delta {
            LocalStateDelta::SetScene(scene) => {
                self.scene = scene;
            },
            LocalStateDelta::SetViewMode(view_mode) => {
                self.view_mode = view_mode;
            },
            LocalStateDelta::ShowExpertView(flag) => {
                self.expert_view = flag;
            },
            LocalStateDelta::ShowLogs(task_id) => {
                self.show_logs_for = Some(task_id);
            },
            LocalStateDelta::HideLogs => {
                self.show_logs_for.take();
            },
        }
        log::debug!("Local state updated: {:?}", self);
    }
}

#[derive(Clone)]
pub enum LocalStateDelta {
    SetScene(Scene),
    SetViewMode(ViewMode),

    ShowExpertView(bool),

    ShowLogs(TaskId),
    HideLogs,
}
