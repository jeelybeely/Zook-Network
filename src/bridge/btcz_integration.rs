// File: src/bridge/btcz_integration.rs

use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::bridge::merkle::MerkleTree;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTCZAnchorPayload {
    pub block_height: u64,
    pub state_root: String,
    pub merkle_proof: Vec<String>,
    pub validator_compliance: bool,
    pub timestamp: DateTime<Utc>,
}

pub struct BTCZIntegration {
    pub rpc_endpoint: String, // Endpoint to BTCZ Core RPC
}

impl BTCZIntegration {
    pub fn new(rpc_endpoint: String) -> Self {
        Self { rpc_endpoint }
    }

    pub async fn send_anchor(&self, payload: BTCZAnchorPayload) -> Result<(), String> {
        // Serialize payload
        let serialized_payload = serde_json::to_string(&payload).map_err(|_| "Serialization failed")?;

        // Make an HTTP POST request to BTCZ Core
        let client = reqwest::Client::new();
        let response = client
            .post(&self.rpc_endpoint)
            .header("Content-Type", "application/json")
            .body(serialized_payload)
            .send()
            .await
            .map_err(|err| format!("Request failed: {}", err))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to send anchor: HTTP {}", response.status()))
        }
    }

    pub fn validate_merkle_proof(
        &self,
        merkle_tree: &MerkleTree,
        transaction_hash: &str,
        proof: &[String],
    ) -> bool {
        merkle_tree.validate_proof(transaction_hash, proof)
    }

    pub fn generate_merkle_proof(&self, merkle_tree: &MerkleTree, transaction_hash: &str) -> Vec<String> {
        merkle_tree.get_proof(transaction_hash)
    }
}
