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

use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Envs, ManagedContainer, Mounts, Networks, Ports, Volumes},
};

use super::GRAFANA_REGISTRY;
use crate::resources::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    images::{GENERAL_VOLUME, GRAFANA_DEFAULTS_PATH, GRAFANA_PROVISION_PATH, GRAFANA_VOLUME, VAR_TARI_PATH},
    networks::LocalNet,
    volumes::SharedGrafanaVolume,
};

#[derive(Debug, Default)]
pub struct Grafana {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for Grafana {
    fn id() -> TaskId {
        "Grafana".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedGrafanaVolume::id()]
    }
}

impl ManagedContainer for Grafana {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        GRAFANA_REGISTRY
    }

    fn image_name(&self) -> &str {
        "grafana"
    }

    fn envs(&self, envs: &mut Envs) {
        let path = concat!(
            "/usr/share/grafana/bin:",
            "/usr/local/sbin:",
            "/usr/local/bin:",
            "/usr/sbin:",
            "/usr/bin:",
            "/sbin:",
            "/bin"
        );
        envs.set("PATH", path);
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Check the `display` call is correct here?
            envs.set("DATA_FOLDER", settings.data_directory.display());
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("grafana", LocalNet::id());
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_300);
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        config.map(|conf| conf.with_monitoring).unwrap_or_default()
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        mounts.add_volume(SharedGrafanaVolume::id(), GRAFANA_VOLUME);
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Avoid using display here
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
            mounts.bind_path(
                settings.data_directory.join("config").join("defaults.ini").display(),
                GRAFANA_DEFAULTS_PATH,
            );
            mounts.bind_path(
                settings
                    .data_directory
                    .join("config")
                    .join("sources_provision.yml")
                    .display(),
                GRAFANA_PROVISION_PATH,
            );
        }
    }
}
