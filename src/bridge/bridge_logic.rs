// File: src/bridge/bridge_logic.rs

use std::collections::HashMap;
use crate::bridge::cross_layer_sync::CrossLayerSync;
use crate::bridge::state_anchoring::{StateAnchoring, L2StateSummary};
use crate::bridge::merkle::MerkleTree;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct BridgeLedger {
    pub locked_tokens: HashMap<String, u64>, // Address and amount locked
    pub burned_tokens: HashMap<String, u64>, // Address and amount burned
    pub state_anchoring: StateAnchoring, // State anchoring for cross-layer sync
    pub cross_layer_sync: CrossLayerSync, // Cross-layer synchronization module
}

impl BridgeLedger {
    pub fn new(state_anchoring: StateAnchoring, cross_layer_sync: CrossLayerSync) -> Self {
        Self {
            locked_tokens: HashMap::new(),
            burned_tokens: HashMap::new(),
            state_anchoring,
            cross_layer_sync,
        }
    }

    pub fn lock_tokens(&mut self, address: String, amount: u64) -> Result<(), String> {
        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        let entry = self.locked_tokens.entry(address.clone()).or_insert(0);
        *entry += amount;

        println!("Tokens locked: {} -> {}", address, *entry);
        Ok(())
    }

    pub fn burn_tokens(&mut self, address: String, amount: u64, transaction_hash: String) -> Result<(), String> {
        let current_balance = self.locked_tokens.get(&address).cloned().unwrap_or(0);

        if amount > current_balance {
            return Err("Burn amount exceeds locked balance".to_string());
        }

        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        self.locked_tokens.insert(address.clone(), current_balance - amount);

        let burn_entry = self.burned_tokens.entry(address.clone()).or_insert(0);
        *burn_entry += amount;

        // Generate Merkle proof for transaction
        let merkle_tree = MerkleTree::new(vec![transaction_hash.clone()]);
        let proof = merkle_tree.get_proof(&transaction_hash);

        // Anchor state to cross-layer sync
        let summary = L2StateSummary {
            block_height: 0, // Placeholder, replace with actual height
            state_root: merkle_tree.get_root(),
            total_transactions: 1,
            timestamp: chrono::Utc::now(),
        };

        self.state_anchoring.anchor_state(summary.clone())?;
        self.cross_layer_sync.anchor_state(summary, true, proof)?;

        println!("Tokens burned: {} -> {}", address, *burn_entry);
        Ok(())
    }

    pub fn get_locked_balance(&self, address: &String) -> u64 {
        *self.locked_tokens.get(address).unwrap_or(&0)
    }

    pub fn get_burned_balance(&self, address: &String) -> u64 {
        *self.burned_tokens.get(address).unwrap_or(&0)
    }

    pub fn audit_token_flow(&self) -> (u64, u64) {
        let total_locked: u64 = self.locked_tokens.values().sum();
        let total_burned: u64 = self.burned_tokens.values().sum();
        (total_locked, total_burned)
    }
}