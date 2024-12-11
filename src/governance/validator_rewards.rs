// File: src/governance/validator_rewards.rs

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::validator::node_registration::ValidatorNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceLog {
    pub timestamp: DateTime<Utc>,
    pub is_compliant: bool,
    pub compliance_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorReward {
    pub address: String,
    pub total_rewards: u64,
    pub last_reward_time: DateTime<Utc>,
    pub compliance_logs: Vec<ComplianceLog>,
}

#[derive(Debug, Clone)]
pub struct ValidatorRewards {
    pub rewards: Arc<Mutex<HashMap<String, ValidatorReward>>>,
    pub reward_rate: u64,
    pub storage_path: PathBuf, // Persistent storage path
}

impl ValidatorRewards {
    pub fn new(reward_rate: u64, storage_path: PathBuf) -> Self {
        let rewards = if storage_path.exists() {
            match fs::read_to_string(&storage_path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => HashMap::new(),
            }
        } else {
            HashMap::new()
        };

        Self {
            rewards: Arc::new(Mutex::new(rewards)),
            reward_rate,
            storage_path,
        }
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        let rewards = self.rewards.lock().map_err(|_| "Mutex lock failed")?;
        let serialized = serde_json::to_string(&*rewards).map_err(|_| "Serialization failed")?;
        fs::write(&self.storage_path, serialized).map_err(|_| "Failed to write to disk")?;
        Ok(())
    }

    pub fn distribute_rewards(&self, validators: &[ValidatorNode]) -> Result<(), String> {
        let mut rewards = self.rewards.lock().map_err(|_| "Mutex lock failed")?;
        let now = Utc::now();

        for validator in validators {
            if !validator.active {
                continue;
            }

            let reward_entry = rewards.entry(validator.address.clone()).or_insert(ValidatorReward {
                address: validator.address.clone(),
                total_rewards: 0,
                last_reward_time: now,
                compliance_logs: Vec::new(),
            });

            let reward = validator.staked_btcz * self.reward_rate / 1_000_000;
            reward_entry.total_rewards += reward;
            reward_entry.last_reward_time = now;

            println!("Distributed reward: {} -> {}", validator.address, reward);
        }

        self.save_to_disk()?;
        Ok(())
    }

    pub fn log_compliance(&self, address: &str, is_compliant: bool, reason: String) -> Result<(), String> {
        let mut rewards = self.rewards.lock().map_err(|_| "Mutex lock failed")?;
        let reward_entry = rewards.get_mut(address).ok_or("Validator not found")?;

        reward_entry.compliance_logs.push(ComplianceLog {
            timestamp: Utc::now(),
            is_compliant,
            compliance_reason: reason.clone(),
        });

        println!("Logged compliance: {} -> {}, Reason: {}", address, is_compliant, reason);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn get_compliance_logs(&self, address: &str) -> Result<Vec<ComplianceLog>, String> {
        let rewards = self.rewards.lock().map_err(|_| "Mutex lock failed")?;
        let reward_entry = rewards.get(address).ok_or("Validator not found")?;
        Ok(reward_entry.compliance_logs.clone())
    }

    pub fn get_total_rewards(&self, address: &str) -> Result<u64, String> {
        let rewards = self.rewards.lock().map_err(|_| "Mutex lock failed")?;
        let reward_entry = rewards.get(address).ok_or("Validator not found")?;
        Ok(reward_entry.total_rewards)
    }
}
