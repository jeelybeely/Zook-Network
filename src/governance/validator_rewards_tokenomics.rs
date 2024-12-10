// File: src/governance/validator_rewards_tokenomics.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorRewardRecord {
    pub address: String,
    pub rewards_earned: u64,
    pub last_reward_time: DateTime<Utc>,
}

pub struct ValidatorRewardsTokenomics {
    pub reward_rate: u64, // Reward rate per cycle in gBTCZ
    pub validator_records: Arc<Mutex<HashMap<String, ValidatorRewardRecord>>>,
}

impl ValidatorRewardsTokenomics {
    pub fn new(reward_rate: u64) -> Self {
        Self {
            reward_rate,
            validator_records: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_validator(&self, address: String) -> Result<(), String> {
        let mut records = self.validator_records.lock().map_err(|_| "Mutex lock failed")?;
        if records.contains_key(&address) {
            return Err("Validator already registered".to_string());
        }

        records.insert(
            address.clone(),
            ValidatorRewardRecord {
                address,
                rewards_earned: 0,
                last_reward_time: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn distribute_rewards(&self) -> Result<(), String> {
        let mut records = self.validator_records.lock().map_err(|_| "Mutex lock failed")?;
        let now = Utc::now();

        for record in records.values_mut() {
            record.rewards_earned += self.reward_rate;
            record.last_reward_time = now;
        }

        println!("Rewards distributed to validators: {:?}", records);
        Ok(())
    }

    pub fn adjust_reward_rate(&mut self, new_rate: u64) -> Result<(), String> {
        self.reward_rate = new_rate;
        println!("Validator reward rate adjusted to: {}", new_rate);
        Ok(())
    }

    pub fn apply_governance_adjustment(&mut self, param: &str, value: u64) -> Result<(), String> {
        match param {
            "reward_rate" => self.adjust_reward_rate(value),
            _ => Err("Unsupported governance parameter".to_string()),
        }
    }

    pub fn distribute_rewards_api(&self) -> String {
        match self.distribute_rewards() {
            Ok(_) => "Rewards distributed successfully".to_string(),
            Err(e) => format!("Error distributing rewards: {}", e),
        }
    }

    pub fn apply_adjustment_api(&mut self, param: &str, value: u64) -> String {
        match self.apply_governance_adjustment(param, value) {
            Ok(_) => format!("Adjustment applied: {} set to {}", param, value),
            Err(e) => format!("Error applying adjustment: {}", e),
        }
    }

    pub fn get_rewards_api(&self, address: &str) -> String {
        match self.get_validator_rewards(address) {
            Some(record) => format!(
                "Validator: {}, Rewards Earned: {}, Last Reward Time: {}",
                record.address, record.rewards_earned, record.last_reward_time
            ),
            None => "Validator not found".to_string(),
        }
    }
}
