use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, From, Into, PartialOrd, Ord, PartialEq, Eq, Hash, Display, Serialize, Deserialize,
)]
pub struct JobId(String);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JobState {}

pub enum JobDelta {}
