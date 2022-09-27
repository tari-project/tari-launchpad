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

use std::convert::TryFrom;

use anyhow::Error;
use tauri::{AppHandle, Manager, Wry};

use crate::{
    commands::ServiceSettings,
    docker::{remove_all_containers, remove_all_volumes, shutdown_all_containers, LaunchpadConfig},
    AppState,
    DEFAULT_WORKSPACE_NAME,
};

#[tauri::command]
pub async fn clean_docker(app: AppHandle<Wry>, settings: ServiceSettings) -> Result<(), String> {
    clean_docker_impl(app, settings).await.map_err(|err| err.to_string())
}

async fn clean_docker_impl(app: AppHandle<Wry>, settings: ServiceSettings) -> Result<(), Error> {
    let state = app.state::<AppState>();
    let config = LaunchpadConfig::try_from(settings)?;
    shutdown_all_containers(DEFAULT_WORKSPACE_NAME, &state.docker)
        .await
        .ok();
    remove_all_containers(DEFAULT_WORKSPACE_NAME, &state.docker).await.ok();
    remove_all_volumes(DEFAULT_WORKSPACE_NAME, config.tari_network, &state.docker)
        .await
        .ok();
    Ok(())
}
