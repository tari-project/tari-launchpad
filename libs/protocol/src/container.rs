// Copyright 2022. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::fmt;

use byte_unit::Byte;
use chrono::NaiveDateTime;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::AsRefStr;

use crate::{errors::ErrorRecord, frame::Frame};

#[derive(Debug, Clone, From, Into, PartialOrd, Ord, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
pub struct TaskId(String);

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl AsRef<str> for TaskId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

const TAIL_LIMIT: usize = 30;

const FAILS_LIMIT: usize = 10;

const STATS_LIMIT: usize = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    pub datetime: NaiveDateTime,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, AsRefStr, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum LogLevel {
    Trace = 0b00001,
    Debug = 0b00010,
    Info = 0b00100,
    Warn = 0b01000,
    Error = 0b10000,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub status: TaskStatus,
    pub tail: Frame<LogRecord>,
    pub fails: Frame<ErrorRecord>,
    pub stats: Frame<StatsData>,
    pub permanent: bool,
}

impl TaskState {
    pub fn new(permanent: bool) -> Self {
        Self {
            status: TaskStatus::Inactive,
            tail: Frame::new(TAIL_LIMIT),
            fails: Frame::new(FAILS_LIMIT),
            stats: Frame::new(STATS_LIMIT),
            permanent,
        }
    }

    pub fn apply(&mut self, delta: TaskDelta) {
        match delta {
            TaskDelta::UpdateStatus(status) => {
                self.status = status;
            },
            TaskDelta::LogRecord(record) => {
                self.tail.push(record);
            },
            TaskDelta::LogError(record) => {
                self.fails.push(record);
            },
            TaskDelta::StatsRecord(record) => {
                self.stats.push(record);
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskProgress {
    pub pct: u8,
    pub stage: String,
}

impl TaskProgress {
    pub fn new(stage: impl ToString) -> Self {
        Self {
            pct: 0,
            stage: stage.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Inactive,
    /// Waiting for dependencies.
    Pending,
    /// Task failed due to missing configuration.
    MissingConfiguration(String),
    /// Task is starting up
    Progress(TaskProgress),
    /// Task is running
    Active,
    /// Task failed for other reasons.
    Failed(String),
}

impl TaskStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_started(&self) -> bool {
        !matches!(self, Self::Inactive)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    pub fn is_missing_configuration(&self) -> Option<&str> {
        match self {
            Self::MissingConfiguration(reason) => Some(reason.as_str()),
            _ => None,
        }
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(_))
    }

    pub fn progress(&self) -> Option<String> {
        match self {
            Self::Progress(progress) => Some(format!("{} -{}%", progress.stage, progress.pct)),
            _ => None,
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "Inactive"),
            Self::Pending => write!(f, "Pending"),
            Self::Progress(value) => write!(f, "Progress({} - {}%)", value.stage, value.pct),
            Self::Active => write!(f, "Active"),
            Self::MissingConfiguration(reason) => write!(f, "Missing configuration. Details: {}", reason),
            Self::Failed(reason) => write!(f, "Failed. Reason: {}", reason),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskDelta {
    UpdateStatus(TaskStatus),
    LogRecord(LogRecord),
    LogError(ErrorRecord),
    StatsRecord(StatsData),
}

impl Frame<StatsData> {
    pub fn last_cpu(&self) -> Option<f32> {
        let mut values = self.iter().rev();
        let last = values.next()?;
        let prev = values.next()?;
        let cpu_delta = last.cpu_usage - prev.cpu_usage;
        let system_delta = last.system_cpu_usage - prev.system_cpu_usage;
        Some((cpu_delta as f32 / system_delta as f32 * 100.0).clamp(0.0, 100.0))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsData {
    pub timestamp: NaiveDateTime,
    pub system_cpu_usage: i64,
    pub cpu_usage: i64,
    pub mem_limit: Byte,
    pub mem_usage: Byte,
}

impl StatsData {
    pub fn get_mem_pct(&self) -> f32 {
        self.mem_usage.as_u128() as f32 * 100.0 / self.mem_limit.as_u128() as f32
    }
}
