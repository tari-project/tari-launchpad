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

use std::{collections::VecDeque, fmt};

use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, From, Into, PartialOrd, Ord, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
pub struct TaskId(String);

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

const TAIL_LIMIT: usize = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub status: TaskStatus,
    pub tail: VecDeque<String>,
}

impl Default for TaskState {
    fn default() -> Self {
        Self {
            status: TaskStatus::Inactive,
            tail: VecDeque::with_capacity(TAIL_LIMIT),
        }
    }
}

impl TaskState {
    pub fn apply(&mut self, delta: TaskDelta) {
        match delta {
            TaskDelta::UpdateStatus(status) => {
                self.status = status;
            },
            TaskDelta::LogRecord(record) => {
                if self.tail.len() == TAIL_LIMIT {
                    self.tail.pop_back();
                }
                self.tail.push_front(record);
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Inactive,
    /// Waiting for dependencies.
    Pending,
    Progress(u8, String),
    Active,
    // TODO: Add failed with a reason?
}

impl TaskStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "Inactive"),
            Self::Pending => write!(f, "Pending"),
            Self::Progress(pct, stage) => write!(f, "Progress({} - {}%)", stage, pct),
            Self::Active => write!(f, "Active"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskDelta {
    UpdateStatus(TaskStatus),
    LogRecord(String),
}
