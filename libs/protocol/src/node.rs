// // Copyright 2023. The Tari Project
// // SPDX-License-Identifier: BSD-3-Clause

use minotari_node_grpc_client::grpc::NodeIdentity;
use serde::{Deserialize, Serialize};
use tari_common_types::{emoji::EmojiId, types::PublicKey};
use tari_utilities::byte_array::ByteArray;

use crate::wallet::InvalidPublicKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseNodeAddress {
    pub public_key: String,
    pub emoji_id: String,
}

impl TryFrom<NodeIdentity> for BaseNodeAddress {
    type Error = InvalidPublicKey;

    fn try_from(value: NodeIdentity) -> Result<Self, Self::Error> {
        let public_key = PublicKey::from_vec(&value.public_key).map_err(|e| InvalidPublicKey(e.to_string()))?;
        let emoji_id = EmojiId::from_public_key(&public_key).to_string();
        Ok(BaseNodeAddress {
            public_key: public_key.to_string(),
            emoji_id,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    // The base node's identity
    pub identity: Option<BaseNodeAddress>,
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
    SetIdentity(BaseNodeAddress),
    SetSyncStatus(String),
    SetPeerCount(usize),
    SetChainLength(u64),
}
