// File: src/validator/node_registration.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ValidatorNode {
    pub address: String,
    pub staked_btcz: u64, // Amount of BTCZ staked
    pub registered_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct ValidatorRegistry {
    pub nodes: Arc<Mutex<HashMap<String, ValidatorNode>>>,
    pub minimum_stake: u64, // Minimum staking requirement
}

impl ValidatorRegistry {
    pub fn new(minimum_stake: u64) -> Self {
        Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
            minimum_stake,
        }
    }

    pub fn register_node(&self, address: String, staked_btcz: u64) -> Result<(), String> {
        if staked_btcz < self.minimum_stake {
            return Err(format!(
                "Insufficient stake. Minimum required is {} BTCZ.",
                self.minimum_stake
            ));
        }

        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        if nodes.contains_key(&address) {
            return Err("Validator already registered".to_string());
        }

        let node = ValidatorNode {
            address: address.clone(),
            staked_btcz,
            registered_at: Utc::now(),
            active: true,
        };

        nodes.insert(address.clone(), node);
        println!("Validator registered: {} with {} BTCZ", address, staked_btcz);
        Ok(())
    }

    pub fn deactivate_node(&self, address: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        let node = nodes.get_mut(address).ok_or("Validator not found")?;
        node.active = false;
        println!("Validator deactivated: {}", address);
        Ok(())
    }

    pub fn update_stake(&self, address: &str, additional_stake: u64) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        let node = nodes.get_mut(address).ok_or("Validator not found")?;
        node.staked_btcz += additional_stake;
        println!("Updated stake for {}: {} BTCZ", address, node.staked_btcz);
        Ok(())
    }

    pub fn list_nodes(&self) -> Result<Vec<ValidatorNode>, String> {
        let nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        Ok(nodes.values().cloned().collect())
    }

    pub fn get_node(&self, address: &str) -> Result<ValidatorNode, String> {
        let nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        nodes.get(address).cloned().ok_or("Validator not found".to_string())
    }
}
