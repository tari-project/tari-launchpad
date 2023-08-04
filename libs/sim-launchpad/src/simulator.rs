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

use std::collections::{HashMap, HashSet};

use anyhow::{anyhow as err, Error};
use chrono::Local;
use rand::Rng;
use tari_launchpad_protocol::{
    container::{LogLevel, LogRecord, StatsData, TaskDelta, TaskId, TaskProgress, TaskState, TaskStatus},
    launchpad::{Action, LaunchpadAction, LaunchpadDelta, LaunchpadState, Reaction},
    wallet::{WalletBalance, WalletDelta},
};
use tari_sdm::ids::ManagedTask;
use tari_sdm_launchpad::resources::images;
use tokio::{
    select,
    sync::mpsc,
    time::{sleep, Duration, Instant},
};

pub struct Simulator {
    in_rx: mpsc::UnboundedReceiver<Action>,
    out_tx: mpsc::UnboundedSender<Reaction>,
    lp_state: LaunchpadState,
    deps: HashMap<TaskId, HashSet<TaskId>>,

    mined_at: Instant,
}

impl Simulator {
    #[tokio::main]
    pub async fn create_and_run(
        in_rx: mpsc::UnboundedReceiver<Action>,
        out_tx: mpsc::UnboundedSender<Reaction>,
    ) -> Result<(), Error> {
        let mut simulator = Self {
            in_rx,
            out_tx,
            lp_state: LaunchpadState::default(),
            deps: HashMap::new(),
            mined_at: Instant::now(),
        };
        simulator.entrypoint().await
    }

    fn init_tasks(&mut self) -> Result<(), Error> {
        self.add_task(images::Tor::id(), vec![])?;
        self.add_task(images::TariBaseNode::id(), vec![images::Tor::id()])?;
        self.add_task(images::TariWallet::id(), vec![images::TariBaseNode::id()])?;
        self.add_task(images::TariSha3Miner::id(), vec![images::TariWallet::id()])?;

        self.add_task(images::Loki::id(), vec![])?;
        self.add_task(images::Promtail::id(), vec![])?;
        self.add_task(images::Grafana::id(), vec![])?;

        self.add_task(images::MmProxy::id(), vec![])?;
        self.add_task(images::Monerod::id(), vec![])?;
        self.add_task(images::XMRig::id(), vec![])?;
        Ok(())
    }

    async fn entrypoint(&mut self) -> Result<(), Error> {
        self.init_tasks()?;
        let mut alive = true;
        while alive {
            match self.step().await {
                Ok(flag) => {
                    alive = flag;
                },
                Err(err) => {
                    println!("{err}");
                },
            }
        }
        Ok(())
    }

    async fn step(&mut self) -> Result<bool, Error> {
        select! {
            action = self.in_rx.recv() => {
                if let Some(Action::Action(action)) = action {
                    self.process_action(action)?;
                } else {
                    return Ok(false);
                }
            }
            _ = sleep(Duration::from_millis(500)) => {
                self.update_state()?;
            }
        }
        Ok(true)
    }

    fn send(&mut self, reaction: Reaction) -> Result<(), Error> {
        self.out_tx
            .send(reaction)
            .map_err(|err| err!("Can't send an event: {err:?}"))
    }

    fn send_state(&mut self) -> Result<(), Error> {
        let state = self.lp_state.clone();
        let reaction = Reaction::State(state);
        self.send(reaction)
    }

    fn apply_delta(&mut self, delta: LaunchpadDelta) -> Result<(), Error> {
        self.lp_state.apply(delta.clone());
        let reaction = Reaction::Delta(delta);
        self.send(reaction)
    }

    fn apply_task_delta(&mut self, id: &TaskId, delta: TaskDelta) -> Result<(), Error> {
        let id = id.clone();
        let delta = LaunchpadDelta::TaskDelta { id, delta };
        self.apply_delta(delta)
    }

    fn apply_wallet_delta(&mut self, delta: WalletDelta) -> Result<(), Error> {
        let delta = LaunchpadDelta::WalletDelta(delta);
        self.apply_delta(delta)
    }

    fn process_action(&mut self, action: LaunchpadAction) -> Result<(), Error> {
        match action {
            LaunchpadAction::Connect => {
                self.send_state()?;
            },
            LaunchpadAction::ChangeSession(session) => {
                self.apply_delta(LaunchpadDelta::UpdateSession(session))?;
            },
        }
        Ok(())
    }

    fn add_task(&mut self, id: TaskId, deps: Vec<TaskId>) -> Result<(), Error> {
        self.deps.insert(id.clone(), deps.into_iter().collect());
        let state = TaskState::new(false);
        let delta = LaunchpadDelta::TaskAdded { id, state };
        self.apply_delta(delta)
    }

    fn update_task(&mut self, id: TaskId, active: bool) -> Result<(), Error> {
        let mut new_status = None;
        let task_status_prev = self
            .lp_state
            .containers
            .get(&id)
            .ok_or_else(|| Error::msg("No task"))?
            .status
            .clone();
        if task_status_prev.is_started() {
            // Add a log record.
            let now = chrono::Local::now();
            // TODO: Put the datetime to the `LogRecord` instead
            let message = format!("{now} - log");
            let record = LogRecord {
                datetime: Local::now().naive_local(),
                level: LogLevel::Info,
                message,
            };
            let delta = TaskDelta::LogRecord(record);
            self.apply_task_delta(&id, delta)?;

            let mut rng = rand::thread_rng();
            let cpu_usage = rng.gen_range(1_000..4_000);
            let stats = StatsData {
                timestamp: chrono::Local::now().naive_local(),
                system_cpu_usage: cpu_usage * rng.gen_range(50..55) / 10,
                cpu_usage,
                mem_limit: (10u64.pow(10)).into(),
                mem_usage: rng.gen_range(10u64.pow(8)..2 * 10u64.pow(8)).into(),
            };
            let delta = TaskDelta::StatsRecord(stats);
            self.apply_task_delta(&id, delta)?;
        }
        match (task_status_prev, active) {
            (TaskStatus::Active, true) => {},
            (TaskStatus::Inactive, true) => {
                let deps = self.deps.get(&id).map(HashSet::iter).into_iter().flatten();
                for dep in deps {
                    let is_active = self
                        .lp_state
                        .containers
                        .get(dep)
                        .ok_or_else(|| Error::msg("No task"))?
                        .status
                        .is_active();
                    if !is_active {
                        return Ok(());
                    }
                }
                new_status = Some(TaskStatus::Pending);
            },
            (TaskStatus::Pending, true) => {
                let progress = TaskProgress::new("Starting...");
                new_status = Some(TaskStatus::Progress(progress));
            },
            (TaskStatus::Progress(mut progress), true) if progress.pct < 100 => {
                progress.pct += 25;
                new_status = Some(TaskStatus::Progress(progress));
            },
            (TaskStatus::Progress(_progress), true) => {
                new_status = Some(TaskStatus::Active);
            },
            (TaskStatus::Inactive, false) => {},
            (_, false) => {
                new_status = Some(TaskStatus::Inactive);
            },
        }
        if let Some(status) = new_status {
            let delta = TaskDelta::UpdateStatus(status);
            self.apply_task_delta(&id, delta)?;
        }
        Ok(())
    }

    fn update_wallet(&mut self) -> Result<(), Error> {
        let task_status = self
            .lp_state
            .containers
            .get(&images::TariWallet::id())
            .ok_or_else(|| Error::msg("No task"))?
            .status
            .clone();
        let new_state = task_status.is_active();
        let current_state = self.lp_state.wallet.active;

        if new_state != current_state {
            let delta = WalletDelta::SetActive(new_state);
            self.apply_wallet_delta(delta)?;
            if new_state {
                let balance = WalletBalance {
                    available: 0,
                    pending_incoming: 0,
                    pending_outgoing: 0,
                };
                let delta = WalletDelta::UpdateBalance(balance);
                self.apply_wallet_delta(delta)?;
            }
        }
        Ok(())
    }

    fn update_mining(&mut self) -> Result<(), Error> {
        if self.mined_at.elapsed() >= Duration::from_secs(10) {
            if let Some(balance) = self.lp_state.wallet.balance.as_mut() {
                balance.available += 1_000;
            }
            // TODO: Add a transaction
            self.mined_at = Instant::now();
        }
        Ok(())
    }

    fn update_state(&mut self) -> Result<(), Error> {
        let session = self.lp_state.config.session.clone();

        self.update_task(images::Tor::id(), session.is_tor_active())?;
        self.update_task(images::TariBaseNode::id(), session.is_base_node_active())?;
        self.update_task(images::TariWallet::id(), session.is_wallet_active())?;
        self.update_task(images::TariSha3Miner::id(), session.is_miner_active())?;

        self.update_task(images::Loki::id(), session.is_loki_active())?;
        self.update_task(images::Promtail::id(), session.is_promtail_active())?;
        self.update_task(images::Grafana::id(), session.is_grafana_active())?;

        self.update_task(images::MmProxy::id(), session.is_mmproxy_active())?;
        self.update_task(images::Monerod::id(), session.is_monerod_active())?;
        self.update_task(images::XMRig::id(), session.is_xmrig_active())?;

        self.update_wallet()?;
        self.update_mining()?;
        Ok(())
    }
}
