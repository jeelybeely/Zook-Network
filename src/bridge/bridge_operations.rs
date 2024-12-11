// File: src/bridge/bridge_operations.rs

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::bridge::merkle::MerkleTree;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockTransaction {
    pub tx_id: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub sender: String,
    pub receiver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnTransaction {
    pub tx_id: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub sender: String,
    pub receiver: String,
}

pub struct BridgeOperations {
    pub lock_transactions: Arc<Mutex<Vec<LockTransaction>>>,
    pub burn_transactions: Arc<Mutex<Vec<BurnTransaction>>>,
}

impl BridgeOperations {
    pub fn new() -> Self {
        Self {
            lock_transactions: Arc::new(Mutex::new(Vec::new())),
            burn_transactions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn lock_tokens(&self, tx: LockTransaction) -> Result<(), String> {
        let mut transactions = self.lock_transactions.lock().map_err(|_| "Mutex lock failed")?;
        transactions.push(tx);
        println!("Locked tokens: {:?}", transactions.last());
        Ok(())
    }

    pub fn burn_tokens(&self, tx: BurnTransaction) -> Result<(), String> {
        let mut transactions = self.burn_transactions.lock().map_err(|_| "Mutex lock failed")?;
        transactions.push(tx);
        println!("Burned tokens: {:?}", transactions.last());
        Ok(())
    }

    pub fn validate_lock_proof(&self, merkle_tree: &MerkleTree, tx_id: &str) -> bool {
        let transactions = self.lock_transactions.lock().unwrap_or_default();
        transactions.iter().any(|tx| tx.tx_id == tx_id && merkle_tree.verify_proof(&tx.tx_id))
    }

    pub fn validate_burn_proof(&self, merkle_tree: &MerkleTree, tx_id: &str) -> bool {
        let transactions = self.burn_transactions.lock().unwrap_or_default();
        transactions.iter().any(|tx| tx.tx_id == tx_id && merkle_tree.verify_proof(&tx.tx_id))
    }
}
