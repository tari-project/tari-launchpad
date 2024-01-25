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

mod common;

use std::{thread, time::Duration};

use anyhow::Error;
use common::TestStateInner;
use tari_launchpad_protocol::container::TaskStatus;
use tari_sdm::ids::{ManagedTask, TaskId};
use tari_sdm_launchpad::resources::images;

#[tokio::test]
async fn test_sdm_mining() -> Result<(), Error> {
    let mut state = TestState::initialize()?;
    let mut done = false;
    while !done {
        done = state.step().await?;
    }
    Ok(())
}

enum Status {
    Init,
    /// Waiting when all containers will be active
    ContainersActivated,
    MiningActivated,
    WaitMining,
    ContainersDeactivated,
}

struct TestState {
    // initial_funds: u64,
    status: Status,
    wallet_containers: Vec<TaskId>,
    mining_containers: Vec<TaskId>,
    inner: TestStateInner,
}

impl TestState {
    fn initialize() -> Result<Self, Error> {
        let inner = TestStateInner::setup(1200)?;
        let wallet_containers = vec![images::Tor::id(), images::TariBaseNode::id()];
        let mining_containers = vec![images::TariSha3Miner::id()];
        Ok(Self {
            // initial_funds: 0,
            status: Status::Init,
            wallet_containers,
            mining_containers,
            inner,
        })
    }

    async fn step(&mut self) -> Result<bool, Error> {
        self.inner.step().await?;
        if self.inner.state.is_some() {
            self.check()
        } else {
            Ok(false)
        }
    }

    fn check(&mut self) -> Result<bool, Error> {
        match self.status {
            Status::Init => {
                self.inner.change_session(|session| {
                    session.tor_active = true;
                    session.base_node_active = true;
                    session.wallet_active = true;
                })?;
                self.status = Status::ContainersActivated;
            },
            Status::ContainersActivated => {
                self.status = Status::MiningActivated;
            },
            Status::MiningActivated => {
                if self
                    .inner
                    .check_containers(&self.mining_containers, TaskStatus::is_active)
                {
                    self.status = Status::WaitMining;
                }
            },
            Status::WaitMining => {
                thread::sleep(Duration::from_millis(500));
                self.status = Status::ContainersDeactivated;
            },
            Status::ContainersDeactivated => {
                if self
                    .inner
                    .check_containers(&self.wallet_containers, TaskStatus::is_inactive)
                    && self
                        .inner
                        .check_containers(&self.mining_containers, TaskStatus::is_inactive)
                {
                    return Ok(true);
                }
            },
        }
        Ok(false)
    }
}
