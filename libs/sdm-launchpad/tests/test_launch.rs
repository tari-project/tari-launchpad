mod common;

use anyhow::Error;
use common::TestStateInner;
use tari_launchpad_protocol::container::TaskStatus;
use tari_sdm::ids::{ManagedTask, TaskId};
use tari_sdm_launchpad::resources::images;

#[tokio::test]
async fn test_sdm_state() -> Result<(), Error> {
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
    ContainersDeactivated,
}

struct TestState {
    status: Status,
    containers: Vec<TaskId>,
    inner: TestStateInner,
}

impl TestState {
    fn initialize() -> Result<Self, Error> {
        let inner = TestStateInner::setup(600)?;
        let containers = vec![
            images::Tor::id(),
            images::TariBaseNode::id(),
            images::TariWallet::id(),
            images::TariSha3Miner::id(),
        ];
        Ok(Self {
            status: Status::Init,
            containers,
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
                    session.base_layer_active = true;
                })?;
                self.status = Status::ContainersActivated;
            },
            Status::ContainersActivated => {
                if self.inner.check_containers(&self.containers, TaskStatus::is_ready) {
                    self.inner.change_session(|session| {
                        session.base_layer_active = false;
                    })?;
                    self.status = Status::ContainersDeactivated;
                }
            },
            Status::ContainersDeactivated => {
                if self.inner.check_containers(&self.containers, TaskStatus::is_inactive) {
                    return Ok(true);
                }
            },
        }
        Ok(false)
    }
}
