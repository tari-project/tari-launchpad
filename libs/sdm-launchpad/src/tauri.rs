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

use anyhow::Error;
use tari_launchpad_protocol::{ACTIONS, REACTIONS};
use tauri::{App, Manager, Wry};

use crate::bus::LaunchpadBus;

pub fn bus_setup(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let bus = LaunchpadBus::start()?;

    let in_tx = bus.incoming;
    let _id = app.listen_global(ACTIONS, move |event| {
        println!("New Event");
        if let Some(payload) = event.payload() {
            let res = serde_json::from_str(payload);
            match res {
                Ok(incoming) => {
                    dbg!(&incoming);
                    log::trace!("Incoming event: {:?}", incoming);
                    if let Err(err) = in_tx.send(incoming) {
                        log::error!("Can't forward an incoming event: {:?}", err);
                    }
                },
                Err(err) => {
                    dbg!("Err");
                    dbg!(&err);
                    log::error!("Can't parse incoming event: {}", err);
                },
            }
        }
    });

    let mut out_rx = bus.outgoing;
    tauri::async_runtime::spawn(async move {
        while let Some(event) = out_rx.recv().await {
            handle.emit_all(REACTIONS, event)?;
        }
        Ok::<(), Error>(())
    });

    Ok(())
}
