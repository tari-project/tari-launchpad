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

use derive_more::Deref;
use tokio::time::Instant;

use crate::task::TaskStatusChecker;

pub struct Fallback<S> {
    pub when: Instant,
    pub next_status: S,
}

#[derive(Deref)]
pub struct SdmStatus<S> {
    name: String,
    #[deref]
    status: S,
    has_work: bool,
    fallback: Option<Fallback<S>>,
}

impl<S: Default> SdmStatus<S> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: S::default(),
            has_work: false,
            fallback: None,
        }
    }
}

impl<S> SdmStatus<S> {
    pub fn get(&self) -> &S {
        &self.status
    }

    pub fn has_work(&self) -> bool {
        self.has_work
    }

    pub fn reset_has_work_flag(&mut self) {
        self.has_work = false;
    }
}

impl<S: TaskStatusChecker> SdmStatus<S> {
    pub fn check_fallback(&mut self) {
        if let Some(fallback) = self.fallback.as_ref() {
            let now = Instant::now();
            if fallback.when < now {
                let fallback = self.fallback.take().unwrap();
                self.set(fallback.next_status);
            }
        }
    }

    pub fn set(&mut self, status: S) {
        log::debug!("Set the new status !{}::status={:?}", self.name, self.status);
        self.status = status;
        self.has_work = true;
        self.fallback = None;
    }

    // pub fn set_fallback(&mut self, fallback: Fallback<S>) {
    // self.fallback = Some(fallback);
    // }
}
