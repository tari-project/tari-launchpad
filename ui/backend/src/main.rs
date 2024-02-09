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

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use anyhow::{Context, Error};
use std::{env, thread, time::Duration};
use tari_launchpad_protocol::{
    launchpad::{Action, LaunchpadAction},
    session::LaunchpadSession,
};
use tari_sdm_assets::configurator::Configurator;
use tari_sdm_launchpad::bus;
use tauri::{Manager, RunEvent};
use tokio::sync::mpsc::UnboundedSender;

fn main() -> Result<(), Error> {
    tauri::async_runtime::block_on(async {
        let mut configurator = Configurator::init().unwrap();
        configurator.init_configuration(false).await.unwrap();

        let workdir = configurator.base_path();
        env::set_current_dir(workdir).unwrap();

        log4rs::init_file("config/log4rs-cli.yml", Default::default())
            .context("Can't read a logs configuration file")
            .unwrap();
    });

    let app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                //  window.close_devtools();
            }

            tari_sdm_launchpad::tauri::bus_setup(app)
        })
        .build(tauri::generate_context!())?;

    app.run(|app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            let bus_requester = app_handle.state::<UnboundedSender<Action>>();
            let mut new_session = LaunchpadSession::default();
            new_session.stop_all();
            bus_requester
                .send(Action::Action(LaunchpadAction::ChangeSession(new_session)))
                .unwrap();

            api.prevent_exit();
            app_handle.listen_global("tari:://reactions", move |event| {
                dbg!(event);
            });
            dbg!("Time to close");
            thread::sleep(Duration::from_secs(3));
            app_handle.exit(1);

            //api.prevent_close();
        },
        _ => {},
    });
    Ok(())
}
