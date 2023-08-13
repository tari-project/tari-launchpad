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

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use anyhow::Error;
use crossterm::event::{poll, read, Event};
use tact::Address;

use crate::dashboard::Dashboard;

pub enum TermEvent {
    Event(Event),
    End,
}

pub struct EventHandle {
    _handle: JoinHandle<Result<(), Error>>,
    interrupted: Arc<AtomicBool>,
}

impl EventHandle {
    pub fn new(addr: Address<Dashboard>) -> Self {
        let interrupted = Arc::new(AtomicBool::new(false));
        let handle = std::thread::spawn({
            let interrupted = interrupted.clone();
            move || -> Result<(), Error> {
                while !interrupted.load(Ordering::Relaxed) {
                    let duration = Duration::from_millis(200);
                    let has_event = poll(duration)?;
                    if has_event {
                        let event = read()?;
                        addr.send(TermEvent::Event(event))?;
                    }
                }
                addr.send(TermEvent::End)?;
                Ok(())
            }
        });
        Self {
            _handle: handle,
            interrupted,
        }
    }

    pub fn interrupt(&mut self) {
        self.interrupted.store(true, Ordering::Relaxed);
    }
}
