// File: src/bridge/mod.rs

pub mod merkle;

use merkle::MerkleTree;
use super::validator::ValidatorState;
use std::sync::Arc;

pub struct BridgeModule {
    pub merkle_tree: MerkleTree,
    pub validator_state: Arc<ValidatorState>,
}

impl BridgeModule {
    pub fn new(transactions: Vec<String>, validator_state: Arc<ValidatorState>) -> Self {
        Self {
            merkle_tree: MerkleTree::new(transactions),
            validator_state,
        }
    }

    pub fn lock_btc(&self, tx_id: &str) {
        println!("BTC locked with transaction ID: {}", tx_id);
    }

    pub fn unlock_btc(&self, tx_id: &str) {
        println!("BTC unlocked with transaction ID: {}", tx_id);
    }

    pub fn generate_proof(&self, tx_id: &str) -> Option<String> {
        if self.merkle_tree.contains(tx_id) {
            Some(self.merkle_tree.generate_proof(tx_id))
        } else {
            None
        }
    }

    pub fn validate_burn(&self, tx_id: &str, amount: u64, timestamp: u64) -> Result<(), String> {
        let record = super::validator::BurnRecord {
            tx_id: tx_id.to_string(),
            amount,
            timestamp,
        };
        self.validator_state.validate_burn(&record)
    }
}