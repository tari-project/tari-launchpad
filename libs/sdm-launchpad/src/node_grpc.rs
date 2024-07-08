// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use std::sync::Arc;

use anyhow::Error;
use minotari_app_grpc::tari_rpc::{
    base_node_client::BaseNodeClient, BaseNodeState, Empty, NetworkStatusResponse, NodeIdentity, TipInfoResponse,
};
use tari_launchpad_protocol::{
    errors::ErrorRecord,
    launchpad::{LaunchpadDelta, LaunchpadDelta::AddError, Reaction},
    node::{BaseNodeIdentity, NodeDelta},
};
use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

pub const BASE_NODE_GRPC_ADDRESS: &str = "http://127.0.0.1:18142";
#[derive(Default, Debug)]
pub struct NodeGrpc {}

impl NodeGrpc {
    pub fn new(out_tx: mpsc::UnboundedSender<Reaction>) -> Self {
        let worker = NodeGrpcWorker::new(out_tx);
        tokio::spawn(worker.entrypoint());
        Self {}
    }
}

pub struct NodeGrpcWorker {
    // The channel that receives updates about state changes in the node's state
    out_tx: mpsc::UnboundedSender<Reaction>,
    // A long-lived connection to the gRPC server. It is lazily initialized.
    client: Option<Arc<Mutex<BaseNodeClient<tonic::transport::Channel>>>>,
}

impl NodeGrpcWorker {
    pub fn new(out_tx: mpsc::UnboundedSender<Reaction>) -> Self {
        Self { out_tx, client: None }
    }

    async fn get_connection(&mut self) -> Result<Arc<Mutex<BaseNodeClient<tonic::transport::Channel>>>, Error> {
        if self.client.is_none() {
            let client = Arc::new(Mutex::new(BaseNodeClient::connect(BASE_NODE_GRPC_ADDRESS).await?));
            self.client = Some(client);
        }
        Ok(Arc::clone(self.client.as_ref().unwrap()))
    }

    /// Carries out the routine work for the Node gRPC worker. It does a few things:
    /// * Polls the base node for the network status
    /// * Fetches the identity
    /// * Fetches the peer count
    async fn routine(&mut self) -> Result<(), Error> {
        let connection = self.get_connection().await?;
        let mut lock = connection.lock().await;
        // Update the node identity
        let id = lock.identify(Empty {}).await?;
        let id = id.into_inner();
        self.process_identity(id);

        // Get the network status & peer count
        let status = lock.get_network_status(Empty {}).await?;
        let status = status.into_inner();
        self.process_network_status(status);

        // Get the chain data
        let sync_info = lock.get_tip_info(Empty {}).await?;
        let sync_info = sync_info.into_inner();
        self.process_sync_info(sync_info);
        Ok(())
    }

    pub async fn entrypoint(mut self) {
        loop {
            let result = self.routine().await;
            if let Err(err) = result {
                log::error!("Node grpc routine failed: {}", err);
            }
            // If there were user-defined actions to send to the node gRPC client, we would recv them here and
            // handle them.
            sleep(std::time::Duration::from_millis(5_000)).await;
        }
    }

    fn send_update(&mut self, delta: NodeDelta) {
        let msg = Reaction::Delta(LaunchpadDelta::NodeDelta(delta));
        if let Err(e) = self.out_tx.send(msg) {
            log::error!("Can't send update for the node: {e}");
        }
    }

    fn send_error<S: Into<String>>(&mut self, error: S) {
        let record = ErrorRecord {
            datetime: chrono::Utc::now().naive_local(),
            message: error.into(),
        };
        let msg = Reaction::Delta(AddError(record));
        if let Err(e) = self.out_tx.send(msg) {
            log::error!("Can't send error for the node: {e}");
        }
    }

    fn process_identity(&mut self, id: NodeIdentity) {
        match BaseNodeIdentity::try_from(id) {
            Ok(id) => {
                let delta = NodeDelta::SetIdentity(id);
                self.send_update(delta);
            },
            Err(err) => {
                log::error!("Failed to parse node identity: {}", err.to_string());
                self.send_error(err);
            },
        }
    }

    fn process_network_status(&mut self, status: NetworkStatusResponse) {
        let delta = NodeDelta::SetPeerCount(status.num_node_connections as usize);
        self.send_update(delta);
    }

    fn process_sync_info(&mut self, sync_info: TipInfoResponse) {
        if let Some(metadata) = sync_info.metadata {
            let delta = NodeDelta::SetChainLength(metadata.best_block_height as u64);
            self.send_update(delta);
        }
        let state = BaseNodeState::from_i32(sync_info.base_node_state);
        let state = match state {
            Some(BaseNodeState::StartUp) => "Starting Up",
            Some(BaseNodeState::HeaderSync) => "Loading headers",
            Some(BaseNodeState::HorizonSync) => "Horizon Sync",
            Some(BaseNodeState::Connecting) => "Connecting",
            Some(BaseNodeState::BlockSync) => "Loading Blocks",
            Some(BaseNodeState::Listening) => "Listening",
            Some(BaseNodeState::SyncFailed) => "Sync Failed",
            None => "Not connected",
        };
        let delta = NodeDelta::SetSyncStatus(state.into());
        self.send_update(delta);
    }
}
