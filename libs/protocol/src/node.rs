// // Copyright 2023. The Tari Project
// // SPDX-License-Identifier: BSD-3-Clause

use minotari_node_grpc_client::grpc::NodeIdentity;
use serde::{Deserialize, Serialize};
use tari_common_types::{emoji::EmojiId, types::PublicKey};
use tari_utilities::{byte_array::ByteArray, hex::Hex};

use crate::wallet::InvalidPublicKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseNodeIdentity {
    pub public_key: Vec<u8>,
    pub public_addresses: Vec<String>,
    pub node_id: Vec<u8>,
    pub emoji_id: String,
}

impl BaseNodeIdentity {
    /// Provide the base node connection string. It is of the form
    /// "0eefb45a4de9484eca74846a4f47d2c8d38e76be1fec63b0112bd00d297c0928::/ip4/13.40.98.39/tcp/18189"
    pub fn connection_string(&self) -> String {
        format!("{}::/dns4/base_node/tcp/18189", self.public_key.to_hex())
    }
}

impl TryFrom<NodeIdentity> for BaseNodeIdentity {
    type Error = InvalidPublicKey;

    fn try_from(value: NodeIdentity) -> Result<Self, Self::Error> {
        let public_key = PublicKey::from_vec(&value.public_key).map_err(|e| InvalidPublicKey(e.to_string()))?;
        let emoji_id = EmojiId::from(&public_key).to_string();
        Ok(BaseNodeIdentity {
            public_key: value.public_key,
            public_addresses: value.public_addresses,
            node_id: value.node_id,
            emoji_id,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    // The base node's identity
    pub identity: Option<BaseNodeIdentity>,
    // The sync status of the base node
    pub sync_status: String,
    // The number of peers connected to the base node
    pub peer_count: usize,
    // The current chain_height of the network
    pub chain_height: u64,
}

impl NodeState {
    pub fn apply(&mut self, delta: NodeDelta) {
        use NodeDelta::*;
        match delta {
            SetIdentity(identity) => {
                self.identity = Some(identity);
            },
            SetSyncStatus(status) => {
                self.sync_status = status;
            },
            SetPeerCount(count) => {
                self.peer_count = count;
            },
            SetChainLength(height) => {
                self.chain_height = height;
            },
        }
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            identity: None,
            sync_status: "Not Connected".to_string(),
            peer_count: 0,
            chain_height: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeDelta {
    SetIdentity(BaseNodeIdentity),
    SetSyncStatus(String),
    SetPeerCount(usize),
    SetChainLength(u64),
}
