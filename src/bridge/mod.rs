// File: src/bridge/mod.rs

pub mod merkle;
pub mod bridge_finalization;

use bridge_finalization::{BridgeFinalization, LockRequest, BurnRequest};
use merkle::MerkleTree;
use super::validator::ValidatorState;
use std::sync::Arc;

pub struct BridgeModule {
    pub merkle_tree: MerkleTree,
    pub finalization: BridgeFinalization,
}

impl BridgeModule {
    pub fn new(transactions: Vec<String>, validator_state: Arc<ValidatorState>) -> Self {
        Self {
            merkle_tree: MerkleTree::new(transactions),
            finalization: BridgeFinalization::new(validator_state),
        }
    }

    pub fn lock_btc(&self, request: LockRequest) -> Result<(), String> {
        println!("Processing BTC lock: {:?}", request);
        self.finalization.lock_btc(request)
    }

    pub fn burn_zbtcz(&self, request: BurnRequest) -> Result<(), String> {
        println!("Processing zBTCZ burn: {:?}", request);
        self.finalization.burn_zbtcz(request)
    }

    pub fn unlock_btc(&self, tx_id: &str) -> Result<LockRequest, String> {
        println!("Processing BTC unlock for tx_id: {}", tx_id);
        self.finalization.unlock_btc(tx_id)
    }

    pub fn mint_zbtcz(&self, tx_id: &str) -> Result<BurnRequest, String> {
        println!("Processing zBTCZ mint for tx_id: {}", tx_id);
        self.finalization.mint_zbtcz(tx_id)
    }

    pub fn generate_proof(&self, tx_id: &str) -> Option<String> {
        if self.merkle_tree.contains(tx_id) {
            Some(self.merkle_tree.generate_proof(tx_id))
        } else {
            None
        }
    }
}
