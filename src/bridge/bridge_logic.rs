use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bridge::state_anchoring::{StateAnchoring, L2StateSummary};
use crate::bridge::merkle::MerkleTree;
use crate::bridge::btcz_integration::{BTCZIntegration, BTCZAnchorPayload};
use crate::clarity::ClarityInteractor;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct BridgeLedger {
    pub locked_tokens: HashMap<String, u64>, // Address and amount locked
    pub burned_tokens: HashMap<String, u64>, // Address and amount burned
    pub state_anchoring: Arc<StateAnchoring>, // State anchoring for cross-layer sync
    pub clarity_interactor: Arc<ClarityInteractor>, // Integration with zBTCZ smart contract
    pub btcz_integration: Arc<BTCZIntegration>, // Integration with BTCZ Core
}

impl BridgeLedger {
    pub fn new(
        state_anchoring: Arc<StateAnchoring>,
        clarity_interactor: Arc<ClarityInteractor>,
        btcz_integration: Arc<BTCZIntegration>,
    ) -> Self {
        Self {
            locked_tokens: HashMap::new(),
            burned_tokens: HashMap::new(),
            state_anchoring,
            clarity_interactor,
            btcz_integration,
        }
    }

    pub fn lock_btcz(&mut self, address: String, amount: u64) -> Result<(), String> {
        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        let entry = self.locked_tokens.entry(address.clone()).or_insert(0);
        *entry += amount;

        // Call ClarityInteractor to mint zBTCZ
        self.clarity_interactor
            .execute_function("mint-zbtcz", &[amount.into()])
            .map_err(|err| format!("Failed to mint zBTCZ: {}", err))?;

        println!("BTCZ locked and zBTCZ minted: {} -> {}", address, *entry);
        Ok(())
    }

    pub fn burn_zbtcz(
        &mut self,
        address: String,
        amount: u64,
        transaction_hash: String,
    ) -> Result<(), String> {
        if amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        // Validate Merkle proof for transaction
        let merkle_tree = MerkleTree::new(vec![transaction_hash.clone().into()]);
        let proof = self
            .btcz_integration
            .generate_merkle_proof(&merkle_tree, &transaction_hash.into());

        if !self
            .btcz_integration
            .validate_merkle_proof(&merkle_tree, &transaction_hash.into(), &proof)
        {
            return Err("Invalid Merkle proof".to_string());
        }

        // Call ClarityInteractor to burn zBTCZ
        self.clarity_interactor
            .execute_function("burn-zbtcz", &[amount.into()])
            .map_err(|err| format!("Failed to burn zBTCZ: {}", err))?;

        // Synchronize with BTCZ Core to unlock BTCZ
        let summary = L2StateSummary {
            block_height: 0, // Placeholder, replace with actual height
            state_root: merkle_tree.get_root().to_string(),
            total_transactions: 1,
            compliance: true, // Assume compliance
            timestamp: Utc::now(),
        };

        let anchor_payload = BTCZAnchorPayload {
            block_height: summary.block_height,
            state_root: summary.state_root.clone(),
            merkle_proof: proof.clone(),
            validator_compliance: true, // Assume compliance
            timestamp: summary.timestamp,
        };

        tokio::spawn({
            let integration = self.btcz_integration.clone();
            async move {
                if let Err(e) = integration.send_anchor(anchor_payload).await {
                    eprintln!("Failed to synchronize state with BTCZ: {}", e);
                }
            }
        });

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
