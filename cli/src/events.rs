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
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use log::trace;
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
                let mut v_pressed = false;
                while !interrupted.load(Ordering::Relaxed) {
                    let duration = Duration::from_millis(200);
                    let has_event = poll(duration)?;
                    if has_event {
                        let event = read()?;
                        trace!(target: "crossterm_events", "EventHandle: KeyEvent '{:?}'", event);
                        match event {
                            // 'Ctrl-v' (to paste) does not have the same behaviour on Windows and Linux. This
                            // difference is evident in events `Event::Key(KeyEvent)`. When a key is pressed on Windows,
                            // it sends a `KeyEvent` with `KeyEventKind::Press` and then a `KeyEvent` with
                            // `KeyEventKind::Release`. On Linux, it sends a `KeyEvent` with `KeyEventKind::Press` only.
                            // The difference with 'Ctrl-v' on Linux is that the buffer is pasted with a series of
                            // `KeyEventKind::Press` events for each character, but on Windows it is pasted with a
                            // series of `KeyEventKind::Press` and `KeyEventKind::Release` events for each character,
                            // however, on Windows the 'v' in the 'Ctrl-v' is accompanied by a `KeyEventKind::Release`
                            // without the preceding `KeyEventKind::Press`. In some cases, the `KeyEventKind::Release`
                            // has a modifier `KeyModifiers::CONTROL` and in other cases it does not. Ultimately, this
                            // determines which key events should be handled as terminal events further down the line.
                            // The approach followed here is that `KeyCode::Char('v')` with modifier
                            // `KeyModifiers::CONTROL` or `KeyCode::Char('v')` with `KeyEventKind::Release` without a
                            // preceding `KeyEventKind::Press` should be ignored. All other cases should be handled as
                            // terminal events.
                            Event::Key(KeyEvent {
                                code,
                                modifiers,
                                kind,
                                state: _,
                            }) => {
                                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                                match code {
                                    KeyCode::Char('v') => {
                                        if kind == KeyEventKind::Press {
                                            v_pressed = true;
                                        }
                                        if ctrl {
                                            // 'Ctrl-v' do nothing
                                            v_pressed = false;
                                            trace!(target: "crossterm_events", "EventHandle: KeyEvent '{:?}' (Ctrl-v, do nothing)", code);
                                        } else if kind == KeyEventKind::Release && !v_pressed {
                                            // 'v' released without being pressed, do nothing
                                            trace!(target: "crossterm_events", "EventHandle: KeyEvent '{:?}' (v released without being pressed, do nothing)", code);
                                        } else {
                                            // Process all other cases
                                            if kind == KeyEventKind::Release && v_pressed {
                                                v_pressed = false;
                                            }
                                            addr.send(TermEvent::Event(event))?;
                                        }
                                    },
                                    _ => addr.send(TermEvent::Event(event))?,
                                }
                            },
                            _ => addr.send(TermEvent::Event(event))?,
                        }
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
