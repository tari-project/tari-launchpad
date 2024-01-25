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

use std::path::PathBuf;

use anyhow::Error;
use log::*;
use tari_launchpad_protocol::{
    container::{TaskDelta, TaskId, TaskProgress, TaskState, TaskStatus},
    launchpad::{Action, LaunchpadAction, LaunchpadDelta, LaunchpadState, Reaction},
    settings::PersistentSettings,
};
use tari_sdm::{ids::ManagedTask, utils::create_password, Report, ReportEnvelope, SdmScope};
use tari_sdm_assets::configurator::Configurator;
use tokio::{select, sync::mpsc};

use crate::{
    node_grpc::NodeGrpc,
    resources::{
        config::{LaunchpadProtocol, LaunchpadSettings},
        images, networks, volumes,
    },
};

pub type BusTx = mpsc::UnboundedSender<Action>;
pub type BusRx = mpsc::UnboundedReceiver<Reaction>;

pub struct LaunchpadBus {
    pub incoming: mpsc::UnboundedSender<Action>,
    pub outgoing: mpsc::UnboundedReceiver<Reaction>,
}

impl LaunchpadBus {
    pub fn start() -> Result<Self, Error> {
        let (in_tx, in_rx) = mpsc::unbounded_channel();
        let (out_tx, out_rx) = mpsc::unbounded_channel();
        std::thread::spawn(move || LaunchpadWorker::create_and_run(in_rx, out_tx));
        Ok(Self {
            incoming: in_tx,
            outgoing: out_rx,
        })
    }
}

pub struct LaunchpadWorker {
    state: LaunchpadState,
    scope: SdmScope<LaunchpadProtocol>,
    in_rx: mpsc::UnboundedReceiver<Action>,
    // TODO: Share the sender with the wallet
    out_tx: mpsc::UnboundedSender<Reaction>,
    node_grpc: Option<NodeGrpc>,
}

impl LaunchpadWorker {
    // TODO: Convert it to an actor
    #[tokio::main]
    async fn create_and_run(
        in_rx: mpsc::UnboundedReceiver<Action>,
        out_tx: mpsc::UnboundedSender<Reaction>,
    ) -> Result<(), Error> {
        // TODO: This should respect the configured network and not be hardcoded
        let mut scope = SdmScope::connect("nextnet")?;
        scope.add_network(networks::LocalNet::default())?;
        scope.add_volume(volumes::SharedVolume::default())?;
        scope.add_volume(volumes::SharedGrafanaVolume::default())?;

        scope.add_image(images::Tor::default())?;
        scope.add_image(images::TariBaseNode::default())?;
        scope.add_image(images::TariSha3Miner::default())?;

        scope.add_image(images::Loki::default())?;
        scope.add_image(images::Promtail::default())?;
        scope.add_image(images::Grafana::default())?;

        scope.add_image(images::MmProxy::default())?;
        scope.add_image(images::XMRig::default())?;

        let state = LaunchpadState::default();

        let worker = LaunchpadWorker {
            state,
            scope,
            in_rx,
            out_tx,
            node_grpc: None,
        };
        worker.entrypoint().await;
        Ok(())
    }

    async fn entrypoint(mut self) {
        self.load_configuration().await.ok();
        // TODO: Watch for the config file changes
        let config = self.state.config.clone();
        self.scope.set_config(Some(config)).ok();
        loop {
            if let Err(err) = self.step().await {
                error!("Bus failed: {}", err);
            }
        }
    }

    /// Attempts to load and parse the settings file based on the given root directory.
    /// The settings file is expected to be at `{root}/config/settings.toml` and be valid TOML.
    async fn load_settings(mut path: PathBuf) -> Option<PersistentSettings> {
        path.push("config");
        path.push("settings.toml");
        if !path.exists() {
            return None;
        }
        let data = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| warn!("Can't read the settings file: {e}"))
            .ok()?;
        toml::from_str::<PersistentSettings>(data.as_str())
            .map_err(|e| error!("{} is not valid TOML. {e}", path.to_string_lossy()))
            .ok()
    }

    async fn load_configuration(&mut self) -> Result<(), Error> {
        let mut configurator = Configurator::init()?;
        let data_directory = configurator.base_path().clone();
        configurator.init_configuration(false).await?;
        let saved_settings = Self::load_settings(data_directory.clone()).await.unwrap_or_else(|| {
            warn!("Can't parse the settings file. Reverting to defaults.");
            PersistentSettings::default()
        });
        let config = LaunchpadSettings {
            data_directory,
            with_monitoring: true,
            tor_control_password: create_password(16),
            saved_settings,
            ..Default::default()
        };
        self.apply_delta(LaunchpadDelta::UpdateConfig(config));
        Ok(())
    }

    async fn step(&mut self) -> Result<(), Error> {
        select! {
            // TODO: Watch config changes here
            action = self.in_rx.recv() => {
                if let Some(action) = action {
                    self.process_incoming(action).await?;
                }
            }
            report = self.scope.recv() => {
                if let Some(report) = report {
                    self.process_report(report).await?;
                }
            }
        }
        Ok(())
    }

    async fn process_incoming(&mut self, incoming: Action) -> Result<(), Error> {
        match incoming {
            Action::Action(action) => self.process_action(action).await,
        }
    }

    async fn process_action(&mut self, action: LaunchpadAction) -> Result<(), Error> {
        match action {
            LaunchpadAction::Connect => {
                let state = self.state.clone();
                self.send(Reaction::State(state));
            },
            LaunchpadAction::ChangeSession(session) => {
                self.apply_delta(LaunchpadDelta::UpdateSession(session));
                let config = self.state.config.clone();
                self.scope.set_config(Some(config))?;
            },
            LaunchpadAction::SaveSettings(settings) => {
                self.save_settings(settings).await?;
            },
        }
        Ok(())
    }

    fn apply_progress_update(&mut self, task_id: &TaskId, progress: &TaskProgress) {
        self.state
            .containers
            .entry(task_id.clone())
            .and_modify(|state| {
                state.status = TaskStatus::Progress(progress.clone());
            })
            .or_insert_with(|| {
                let mut state = TaskState::new(false);
                state.status = TaskStatus::Progress(progress.clone());
                state
            });
    }

    fn apply_delta(&mut self, delta: LaunchpadDelta) {
        self.state.apply(delta.clone());
        let reaction = Reaction::Delta(delta);
        self.send(reaction);
    }

    fn send(&mut self, out: Reaction) {
        if let Err(err) = self.out_tx.send(out) {
            error!("Can't send an outgoing message: {}", err);
        }
    }

    async fn save_settings(&mut self, new_settings: PersistentSettings) -> Result<(), Error> {
        debug!("Saving the settings");
        let mut path = self
            .state
            .config
            .settings
            .as_ref()
            .map(|s| s.data_directory.clone())
            .ok_or_else(|| Error::msg("Can't save the settings: no settings are attached to the config"))?;
        path.push("config");
        path.push("settings.toml");
        debug!("Stored settings: {new_settings:?}");
        let data = toml::to_string(&new_settings).unwrap();
        tokio::fs::write(path, data).await?;
        if let Some(settings) = self.state.config.settings.as_mut() {
            // We just checked that this exists above
            settings.saved_settings = new_settings
        }
        Ok(())
    }

    async fn process_report(&mut self, report: ReportEnvelope<LaunchpadProtocol>) -> Result<(), Error> {
        // TODO: Convert to the `LaunchpadDelta` and apply
        match report.details {
            Report::State(state) => {
                let state = LaunchpadDelta::TaskAdded {
                    id: report.task_id,
                    state,
                };
                self.apply_delta(state);
            },
            Report::Delta(delta) => {
                if report.task_id == images::TariBaseNode::id() {
                    self.check_node_grpc(&delta);
                }
                if let TaskDelta::UpdateStatus(TaskStatus::Progress(progress)) = &delta {
                    self.apply_progress_update(&report.task_id, progress);
                }
                let delta = LaunchpadDelta::TaskDelta {
                    id: report.task_id,
                    delta,
                };
                self.apply_delta(delta);
            },
            Report::Extras(_) => {},
        }
        Ok(())
    }

    // Only called if the task is the base node task
    fn check_node_grpc(&mut self, delta: &TaskDelta) {
        if let TaskDelta::UpdateStatus(status) = delta {
            if status.is_active() {
                if self.node_grpc.is_none() {
                    let grpc = NodeGrpc::new(self.out_tx.clone());
                    self.node_grpc = Some(grpc);
                }
            } else {
                // Detaches grpc instances that closes a channel
                if self.node_grpc.is_some() {
                    self.node_grpc.take();
                }
            }
        }
    }
}
