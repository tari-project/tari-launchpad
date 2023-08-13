// Copyright 2023. The Tari Project
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

#[derive(Debug, Clone)]
pub struct Progress {
    pub activity: Option<String>,
    pub pct: u8,
}

impl Progress {
    pub fn new() -> Self {
        Self { activity: None, pct: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Onboarding {
    pub history: Vec<Message>,
    pub message: Option<Message>,
    pub total_progress: Progress,
    pub local_progress: Option<Progress>,
}

impl Onboarding {
    pub fn update(&mut self, delta: OnboardingDelta) {
        match delta {
            OnboardingDelta::Add(msg) => {
                if let Some(message) = self.message.take() {
                    self.history.push(message);
                }
                self.message = Some(msg);
            },
            OnboardingDelta::SetProgress(pct) => {
                self.total_progress.pct = pct;
            },
        }
    }

    pub fn is_done(&self) -> bool {
        self.total_progress.pct == 100
    }
}

impl Default for Onboarding {
    fn default() -> Self {
        Self {
            history: Vec::new(),
            message: None,
            total_progress: Progress::new(),
            local_progress: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OnboardingAction {
    Next,
}

#[derive(Debug, Clone)]
pub enum OnboardingDelta {
    Add(Message),
    SetProgress(u8),
}
