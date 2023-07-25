use crate::job::{JobId, JobState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub jobs: HashMap<JobId, JobState>,
}

pub enum LpDelta {}

pub enum LpAction {}
