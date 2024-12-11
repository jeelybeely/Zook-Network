// File: src/bridge/bridge_logic.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bridge::state_anchoring::{StateAnchoring, L2StateSummary};
use crate::bridge::merkle::MerkleTree;
use crate::clarity::ClarityContract;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct BridgeLedger {
    pub locked_tokens: HashMap<String, u64>, // Address and amount locked
    pub burned_tokens: HashMap<String, u64>, // Address and amount burned
    pub state_anchoring: StateAnchoring, // State anchoring for cross-layer sync
    pub clarity_contract: Arc<ClarityContract>, // Integration with zBTCZ smart contract
}

impl BridgeLedger {
    pub fn new(state_anchoring: StateAnchoring, clarity_contract: Arc<ClarityContract>) -> Self {
        Self {
            locked_tokens: HashMap::new(),
            burned_tokens: HashMap::new(),
            state_anchoring,
            clarity_contract,
        }
    }

    pub fn lock_btcz(&mut self, address: String, amount: u64) -> Result<(), String> {
        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        let entry = self.locked_tokens.entry(address.clone()).or_insert(0);
        *entry += amount;

        // Call Clarity contract to mint zBTCZ
        self.clarity_contract.call("mint-zbtcz", &[amount.into()])
            .map_err(|err| format!("Failed to mint zBTCZ: {}", err))?;

        println!("BTCZ locked and zBTCZ minted: {} -> {}", address, *entry);
        Ok(())
    }

    pub fn burn_zbtcz(&mut self, address: String, amount: u64, transaction_hash: String) -> Result<(), String> {
        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        // Validate Merkle proof for transaction
        let merkle_tree = MerkleTree::new(vec![transaction_hash.clone()]);
        let proof = merkle_tree.get_proof(&transaction_hash);

        // Call Clarity contract to burn zBTCZ
        self.clarity_contract.call("burn-zbtcz", &[amount.into()])
            .map_err(|err| format!("Failed to burn zBTCZ: {}", err))?;

        // Synchronize with BTCZ Core to unlock BTCZ
        let summary = L2StateSummary {
            block_height: 0, // Placeholder, replace with actual height
            state_root: merkle_tree.get_root(),
            total_transactions: 1,
            timestamp: chrono::Utc::now(),
        };

        self.state_anchoring.anchor_state(summary.clone())?;

        println!("zBTCZ burned and BTCZ unlocked: {} -> {}", address, amount);
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
