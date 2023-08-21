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
use tari_launchpad_protocol::{
    container::{TaskDelta, TaskId},
    launchpad::{Action, LaunchpadAction, LaunchpadDelta, LaunchpadState, Reaction},
    settings::WalletConfig,
};
use tari_sdm::{ids::ManagedTask, Report, ReportEnvelope, SdmScope};
use tari_sdm_assets::configurator::Configurator;
use tokio::{select, sync::mpsc};

use crate::{
    resources::{
        config::{LaunchpadProtocol, LaunchpadSettings},
        images,
        networks,
        volumes,
    },
    wallet_grpc::WalletGrpc,
};

pub type BusTx = mpsc::UnboundedSender<Action>;
pub type BusRx = mpsc::UnboundedReceiver<Reaction>;

pub struct LaunchpadBus {
    // pub handle: JoinHandle<()>,
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
    wallet_task_id: TaskId,
    wallet_grpc: Option<WalletGrpc>,
}

impl LaunchpadWorker {
    // TODO: Convert it to an actor
    #[tokio::main]
    async fn create_and_run(
        in_rx: mpsc::UnboundedReceiver<Action>,
        out_tx: mpsc::UnboundedSender<Reaction>,
    ) -> Result<(), Error> {
        let mut scope = SdmScope::connect("esmeralda")?;
        scope.add_network(networks::LocalNet::default())?;
        scope.add_volume(volumes::SharedVolume::default())?;
        scope.add_volume(volumes::SharedGrafanaVolume::default())?;

        scope.add_image(images::Tor::default())?;
        scope.add_image(images::TariBaseNode::default())?;
        scope.add_image(images::TariWallet::default())?;
        scope.add_image(images::TariSha3Miner::default())?;

        scope.add_image(images::Loki::default())?;
        scope.add_image(images::Promtail::default())?;
        scope.add_image(images::Grafana::default())?;

        scope.add_image(images::MmProxy::default())?;
        scope.add_image(images::Monerod::default())?;
        scope.add_image(images::XMRig::default())?;

        let state = LaunchpadState::default();

        let worker = LaunchpadWorker {
            state,
            scope,
            in_rx,
            out_tx,
            wallet_task_id: images::TariWallet::id(),
            wallet_grpc: None,
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
                log::error!("Bus failed: {}", err);
            }
        }
    }

    async fn load_configuration(&mut self) -> Result<(), Error> {
        let mut configurator = Configurator::init()?;
        let data_directory = configurator.base_path().clone();
        configurator.init_configuration().await?;
        let wallet_config = WalletConfig {
            password: "123".to_string(),
        };
        let config = LaunchpadSettings {
            data_directory,
            with_monitoring: true,
            tor_control_password: "tari".to_string(), // create_password(16).into(),
            wallet: Some(wallet_config),
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
        }
        Ok(())
    }

    fn apply_delta(&mut self, delta: LaunchpadDelta) {
        self.state.apply(delta.clone());
        let reaction = Reaction::Delta(delta);
        self.send(reaction);
    }

    fn send(&mut self, out: Reaction) {
        if let Err(err) = self.out_tx.send(out) {
            log::error!("Can't send an outgoing message: {}", err);
        }
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
                if report.task_id == self.wallet_task_id {
                    self.check_wallet_grpc(&delta);
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

    fn check_wallet_grpc(&mut self, delta: &TaskDelta) {
        if let TaskDelta::UpdateStatus(status) = delta {
            if status.is_active() {
                if self.wallet_grpc.is_none() {
                    let grpc = WalletGrpc::new(self.out_tx.clone());
                    self.wallet_grpc = Some(grpc);
                }
            } else {
                // Detaches grpc instances that closes a channel
                if self.wallet_grpc.is_some() {
                    self.wallet_grpc.take();
                    // TODO: Send a delta about grpc
                }
            }
        }
    }
}
