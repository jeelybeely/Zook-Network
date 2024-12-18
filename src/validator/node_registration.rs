use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorNode {
    pub address: String,
    pub staked_btcz: u64, // Amount of BTCZ staked
    pub registered_at: DateTime<Utc>,
    pub active: bool,
    pub activity_percentage: u64, // Validator activity percentage
    pub compliance_percentage: u64, // Compliance percentage for governance
    pub voting_power: u64, // Calculated voting power
}

#[derive(Debug, Clone)]
pub struct ValidatorRegistry {
    pub nodes: Arc<Mutex<HashMap<String, ValidatorNode>>>,
    pub minimum_stake: u64, // Minimum staking requirement
    pub storage_path: PathBuf, // Persistent storage path
}

impl ValidatorRegistry {
    pub fn new(minimum_stake: u64, storage_path: PathBuf) -> Self {
        let nodes = if storage_path.exists() {
            match fs::read_to_string(&storage_path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => HashMap::new(),
            }
        } else {
            HashMap::new()
        };

        Self {
            nodes: Arc::new(Mutex::new(nodes)),
            minimum_stake,
            storage_path,
        }
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        let nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        let serialized = serde_json::to_string(&*nodes).map_err(|_| "Serialization failed")?;
        fs::write(&self.storage_path, serialized).map_err(|_| "Failed to write to disk")?;
        Ok(())
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
            activity_percentage: 100, // Default to full activity
            compliance_percentage: 100, // Default to full compliance
            voting_power: staked_btcz, // Initial voting power based on stake
        };

        nodes.insert(address.clone(), node);
        self.save_to_disk()?;
        println!("Validator registered: {} with {} BTCZ", address, staked_btcz);
        Ok(())
    }

    pub fn deactivate_node(&self, address: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        let node = nodes.get_mut(address).ok_or("Validator not found")?;
        node.active = false;
        self.save_to_disk()?;
        println!("Validator deactivated: {}", address);
        Ok(())
    }

    pub fn update_stake(&self, address: &str, additional_stake: u64) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        let node = nodes.get_mut(address).ok_or("Validator not found")?;
        node.staked_btcz += additional_stake;
        node.voting_power += additional_stake; // Update voting power with additional stake
        self.save_to_disk()?;
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

    pub fn update_activity_and_compliance(
        &self,
        address: &str,
        activity_percentage: u64,
        compliance_percentage: u64,
    ) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        let node = nodes.get_mut(address).ok_or("Validator not found")?;
        node.activity_percentage = activity_percentage;
        node.compliance_percentage = compliance_percentage;
        self.save_to_disk()?;
        println!(
            "Updated activity and compliance for {}: Activity {}%, Compliance {}%",
            address, activity_percentage, compliance_percentage
        );
        Ok(())
    }
}
