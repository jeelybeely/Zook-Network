// File: src/bridge/mod.rs

pub mod merkle;

use merkle::MerkleTree;

pub struct BridgeModule {
    pub merkle_tree: MerkleTree,
}

impl BridgeModule {
    pub fn new(transactions: Vec<String>) -> Self {
        Self {
            merkle_tree: MerkleTree::new(transactions),
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
}
