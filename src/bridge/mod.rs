// File: src/bridge/mod.rs

pub mod merkle;
pub mod bridge_finalization;
pub mod cross_layer_sync;

use bridge_finalization::{BridgeFinalization, LockRequest, BurnRequest};
use cross_layer_sync::{CrossLayerSync, LockEvent, BurnEvent};
use merkle::MerkleTree;
use super::validator::ValidatorState;
use std::sync::Arc;

pub struct BridgeModule {
    pub merkle_tree: MerkleTree,
    pub finalization: BridgeFinalization,
    pub cross_layer_sync: CrossLayerSync,
}

impl BridgeModule {
    pub fn new(transactions: Vec<String>, validator_state: Arc<ValidatorState>) -> Self {
        Self {
            merkle_tree: MerkleTree::new(transactions),
            finalization: BridgeFinalization::new(validator_state.clone()),
            cross_layer_sync: CrossLayerSync::new(),
        }
    }

    pub fn lock_btc(&self, request: LockRequest) -> Result<(), String> {
        println!("Processing BTCZ lock: {:?}", request);
        self.finalization.lock_btc(request.clone())?;

        let lock_event = LockEvent {
            tx_id: request.tx_id,
            amount: request.amount,
            from_address: request.from_address,
            to_address: request.to_address,
            timestamp: chrono::Utc::now(),
        };
        self.cross_layer_sync.record_lock_event(lock_event)
    }

    pub fn burn_zbtcz(&self, request: BurnRequest) -> Result<(), String> {
        println!("Processing zBTCZ burn: {:?}", request);
        self.finalization.burn_zbtcz(request.clone())?;

        let burn_event = BurnEvent {
            tx_id: request.tx_id,
            amount: request.amount,
            from_address: request.from_address,
            to_address: request.to_address,
            timestamp: chrono::Utc::now(),
        };
        self.cross_layer_sync.record_burn_event(burn_event)
    }

    pub fn unlock_btc(&self, tx_id: &str) -> Result<LockRequest, String> {
        println!("Processing BTCZ unlock for tx_id: {}", tx_id);
        self.finalization.unlock_btc(tx_id)
    }

    pub fn mint_zbtcz(&self, tx_id: &str) -> Result<BurnRequest, String> {
        println!("Processing zBTCZ mint for tx_id: {}", tx_id);
        self.finalization.mint_zbtcz(tx_id)
    }

    pub fn generate_proof(&self, tx_id: &str, event_type: &str) -> Result<String, String> {
        self.cross_layer_sync.generate_merkle_proof(tx_id, event_type)
    }
}
