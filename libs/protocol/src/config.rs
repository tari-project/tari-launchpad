use serde::{Deserialize, Serialize};

use crate::{session::LaunchpadSession, settings::LaunchpadSettings};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaunchpadConfig {
    pub session: LaunchpadSession,
    pub settings: Option<LaunchpadSettings>,
}
