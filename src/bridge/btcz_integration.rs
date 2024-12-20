use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::bridge::merkle::MerkleTree;
use reqwest::Client;
use tokio::time::{sleep, Duration};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTCZAnchorPayload {
    pub block_height: u64,
    pub state_root: String,
    pub merkle_proof: Vec<String>,
    pub validator_compliance: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct BTCZIntegration {
    pub rpc_endpoint: String, // Endpoint to BTCZ Core RPC
}

impl BTCZIntegration {
    pub fn new(rpc_endpoint: String) -> Self {
        Self { rpc_endpoint }
    }

    pub async fn send_anchor(&self, payload: BTCZAnchorPayload) -> Result<(), String> {
        let serialized_payload = serde_json::to_string(&payload)
            .map_err(|err| format!("Serialization failed: {}", err))?;

        let client = Client::new();
        let max_retries = 3;
        let mut retries = 0;

        while retries < max_retries {
            let response = client
                .post(&self.rpc_endpoint)
                .header("Content-Type", "application/json")
                .body(serialized_payload.clone())
                .send()
                .await;

            match response {
                Ok(resp) if resp.status().is_success() => {
                    return Ok(());
                }
                Ok(resp) => {
                    eprintln!(
                        "HTTP Error {}: {:?}",
                        resp.status(),
                        resp.text().await.unwrap_or_default()
                    );
                }
                Err(err) => {
                    eprintln!("Request Error: {}", err);
                }
            }

            retries += 1;
            eprintln!("Retrying... attempt {}/{}", retries, max_retries);
            sleep(Duration::from_secs(2_u64.pow(retries))).await;
        }

        Err("Failed to send anchor after multiple attempts".to_string())
    }

    pub fn validate_merkle_proof(
        &self,
        merkle_tree: &MerkleTree,
        transaction_hash: &str,
        proof: &[String],
    ) -> bool {
        // Corrected: Using verify_proof method instead of nonexistent validate_proof
        let transaction_hash_bytes = transaction_hash.as_bytes().to_vec();
        let proof_bytes: Vec<Vec<u8>> = proof
            .iter()
            .map(|p| hex::decode(p).unwrap_or_default())
            .collect();
        MerkleTree::verify_proof(proof_bytes, merkle_tree.get_root(), transaction_hash_bytes)
    }

    pub fn generate_merkle_proof(
        &self,
        merkle_tree: &MerkleTree,
        transaction_hash: &str,
    ) -> Vec<String> {
        merkle_tree
            .get_proof(&transaction_hash.as_bytes().to_vec())
            .into_iter()
            .map(|hash| hex::encode(hash))
            .collect()
    }
}
