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

use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{anyhow, Error};
use keyring::Entry;
use log::*;
use tari_common_types::tari_address::TariAddress;
use tari_key_manager::cipher_seed::CipherSeed;
use tari_key_manager::mnemonic::{Mnemonic, MnemonicLanguage};
use tari_utilities::SafePassword;
use tari_launchpad_protocol::{
    container::{TaskDelta, TaskId, TaskProgress, TaskState, TaskStatus},
    launchpad::{Action, LaunchpadAction, LaunchpadDelta, LaunchpadState, Reaction},
    settings::PersistentSettings,
};
use tari_sdm::{ids::ManagedTask, utils::create_password, Report, ReportEnvelope, SdmScope};
use tari_sdm_assets::configurator::Configurator;
use tokio::{fs, select, sync::mpsc};

use crate::{
    node_grpc::NodeGrpc,
    resources::{
        config::{LaunchpadProtocol, LaunchpadSettings},
        images, networks, volumes,
    },
};
use rand::Rng;
use tari_common::configuration::Network;
use tari_common_types::types::PublicKey;
use tari_key_manager::key_manager::KeyManager;
use tari_key_manager::key_manager_service::KeyDigest;
use tokio::task::JoinHandle;
use tari_core::transactions::key_manager::{create_memory_db_key_manager, create_memory_db_key_manager_from_seed};

pub type BusTx = mpsc::UnboundedSender<Action>;
pub type BusRx = mpsc::UnboundedReceiver<Reaction>;

const LOG_TARGET: &'static str = "tari::launchpad::sdm::bus";
const KEY_MANAGER_COMMS_SECRET_KEY_BRANCH_KEY: &str = "comms";

pub struct LaunchpadBus {
    pub incoming: mpsc::UnboundedSender<Action>,
    pub outgoing: mpsc::UnboundedReceiver<Reaction>,
    worker_thread: std::thread::JoinHandle<Result<(), Error>>
}

impl LaunchpadBus {
    pub fn start() -> Result<Self, Error> {
        let (in_tx, in_rx) = mpsc::unbounded_channel();
        let (out_tx, out_rx) = mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn(move || LaunchpadWorker::create_and_run(in_rx, out_tx));
        Ok(Self {
            incoming: in_tx,
            outgoing: out_rx,
            worker_thread
        })
    }

    pub fn join(self) -> Result<(), Error> {
        self.worker_thread.join().map_err(|e| anyhow!("Worker thread failed: {:?}", e))?
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
        match self.load_configuration().await {
            Ok(_) => info!(target: LOG_TARGET, "Configuration loaded"),
            Err(err) => {
                dbg!(&err);
                error!(target: LOG_TARGET, "Can't load the configuration: {}", err);

                return;

            },
        }
        // TODO: Watch for the config file changes
        let config = self.state.config.clone();
        match self.scope.set_config(Some(config)) {
            Ok(_) => info!(target: LOG_TARGET, "Configuration set"),
            Err(err) => {
                error!(target: LOG_TARGET, "Can't set the configuration: {}", err);
                return;
            },
        }
        loop {
            if let Err(err) = self.step().await {
                error!(target: LOG_TARGET, "Bus failed: {}", err);
                return;
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
            .map_err(|e| warn!("{} is not valid TOML. {e}", path.to_string_lossy()))
            .ok()
    }

    async fn load_configuration(&mut self) -> Result<(), Error> {
        let mut configurator = Configurator::init()?;
        let data_directory = configurator.base_path().clone();
        configurator.init_configuration(false).await?;
        let mut saved_settings = Self::load_settings(data_directory.clone()).await.unwrap_or_else(|| {
            warn!("Can't parse the settings file. Reverting to defaults.");
            PersistentSettings::default()
        });
        dbg!(&saved_settings);
        if saved_settings.sha3_miner.is_none() || saved_settings.sha3_miner.as_ref().unwrap().wallet_payment_address.is_none()
            || saved_settings.sha3_miner.as_ref().unwrap().wallet_payment_address.as_ref().unwrap().is_empty() {
            warn!("No wallet payment address found in the settings. Generating a new one.");
            let mut base = saved_settings.sha3_miner.as_ref().unwrap_or(&Default::default()).clone();

            let addr = Self::generate_and_save_internal_wallet(data_directory.as_path()).await?;
            base.wallet_payment_address = Some(addr.to_hex());
            saved_settings.sha3_miner = Some(base);
        }
        dbg!("here2");
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

    async fn generate_and_save_internal_wallet(data_dir: &Path) -> Result<TariAddress, Error>{
        let entry = Entry::new("com.tari.launchpad", "internal_wallet")?;

        dbg!("h1");
        let passphrase  = match entry.get_password() {
            Ok(pass) => SafePassword::from_str(&pass).expect("Can't create safe password"),
            Err(_) => {
                dbg!("h1.1");
                // TODO: better generation
                let mut random = rand::thread_rng();
                let mut pass = "".to_string();
                for _ in 0..100 {
                    pass = pass + &random.gen_range(0..10).to_string();
                }
                entry.set_password(&pass)?;
                let pass = SafePassword::from_str(&pass).expect("Can't create safe password");
                pass
            }
        };
        // let seed = CipherSeed::new();
        dbg!("h2");
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).await?;
        }
        dbg!("h3");
        let seed;
        dbg!(data_dir.join("seed_data.encrypted"));
        if fs::try_exists(data_dir.join("seed_data.encrypted")).await? {
            let seed_file = fs::read(data_dir.join("seed_data.encrypted")).await?;
            seed = CipherSeed::from_enciphered_bytes(&seed_file, Some(passphrase))?;
            let seed_words = seed.to_mnemonic(MnemonicLanguage::English, None)?;
            for i in 0..seed_words.len() {
                println!("{}: {}", i + 1, seed_words.get_word(i)?);
            }
        }
        else {
            dbg!("h4");
            seed = CipherSeed::new();
            let seed_file = seed.encipher(Some(passphrase))?;
            fs::write(data_dir.join("seed_data.encrypted"), seed_file).await?;
            let seed_words = seed.to_mnemonic(MnemonicLanguage::English, None)?;
            for i in 0..seed_words.len() {
                println!("{}: {}", i + 1, seed_words.get_word(i)?);
            }
        }
        dbg!("here");

        //Err(anyhow!("Not implemented"))
        // let seed_words = seed.to_mnemonic(MnemonicLanguage::English, None)?;
        // for i in 0..seed_words.len() {
        //     println!("{}: {}", i + 1, seed_words.get_word(i)?);
        // }


        let comms_key_manager = KeyManager::<PublicKey, KeyDigest>::from(
            seed.clone(),
            KEY_MANAGER_COMMS_SECRET_KEY_BRANCH_KEY.to_string(),
            0,
        );
        todo!()
        // let comms_key = comms_key_manager.derive_key(0)?.key;
        // let comms_pub_key = PublicKey::from_secret_key(&comms_key);
        // let network = Network::default();
        //
        //  let tx_key_manager =create_memory_db_key_manager_from_seed(seed.clone(), 64);
        // let view_key = tx_key_manager.get_view_key_id().await?;
        // let view_key_pub = tx_key_manager.get_public_key_at_key_id(&view_key).await?;
        // let tari_address =
        //     TariAddress::new_dual_address_with_default_features(view_key_pub.clone(), comms_pub_key.clone(), network);
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
                self.save_settings(*settings).await?;
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
        let data = toml::to_string(&new_settings).map_err(|e| Error::msg(format!("Can't save the settings: {e}")))?;
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
