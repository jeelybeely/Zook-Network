// File: src/bridge/mod.rs

pub mod merkle;
pub mod bridge_finalization;
pub mod cross_layer_sync;
pub mod state_anchoring;
pub mod btcz_integration;

use bridge_finalization::{BridgeFinalization, LockRequest, BurnRequest};
use cross_layer_sync::{CrossLayerSync, LockEvent, BurnEvent};
use state_anchoring::{StateAnchoring, generate_state_summary, L2StateSummary};
use merkle::MerkleTree;
use super::validator::ValidatorState;
use std::sync::Arc;

pub struct BridgeModule {
    pub merkle_tree: MerkleTree,
    pub finalization: BridgeFinalization,
    pub cross_layer_sync: CrossLayerSync,
    pub state_anchoring: StateAnchoring,
}

impl BridgeModule {
    pub fn new(transactions: Vec<String>, validator_state: Arc<ValidatorState>) -> Self {
        Self {
            merkle_tree: MerkleTree::new(transactions),
            finalization: BridgeFinalization::new(validator_state.clone()),
            cross_layer_sync: CrossLayerSync::new(),
            state_anchoring: StateAnchoring::new(),
        }
    }

    pub fn lock_btcz(&self, request: LockRequest) -> Result<(), String> {
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
        self.finalization.burn_btc(request.clone())?;

        let burn_event = BurnEvent {
            tx_id: request.tx_id,
            amount: request.amount,
            from_address: request.from_address,
            to_address: request.to_address,
            timestamp: chrono::Utc::now(),
        };
        self.cross_layer_sync.record_burn_event(burn_event)
    }

    pub fn anchor_l2_state(&self, block_height: u64, total_transactions: u64) -> Result<L2StateSummary, String> {
        let summary = generate_state_summary(&self.merkle_tree, block_height, total_transactions);
        self.state_anchoring.anchor_state(summary.clone())?;
        println!("Anchored L2 state: {:?}", summary);
        Ok(summary)
    }

    pub fn validate_l2_state(&self, state_root: &str) -> bool {
        self.state_anchoring.validate_anchored_state(state_root)
    }
}
