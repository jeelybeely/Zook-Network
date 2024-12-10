// File: src/governance/validator_rewards.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub locked_btcz: u64,
    pub last_reward_time: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub validator_rewards: HashMap<String, u64>, // Validator address to reward amount
}

pub struct ValidatorRewards {
    pub validators: Arc<Mutex<HashMap<String, Validator>>>,
    pub reward_rate: u64, // Reward per block in gBTCZ
}

impl ValidatorRewards {
    pub fn new(reward_rate: u64) -> Self {
        Self {
            validators: Arc::new(Mutex::new(HashMap::new())),
            reward_rate,
        }
    }

    pub fn register_validator(&self, address: String, locked_btcz: u64) -> Result<(), String> {
        let mut validators = self.validators.lock().map_err(|_| "Mutex lock failed")?;
        if validators.contains_key(&address) {
            return Err("Validator already registered".to_string());
        }

        let validator = Validator {
            address: address.clone(),
            locked_btcz,
            last_reward_time: Utc::now(),
            active: true,
        };

        validators.insert(address, validator);
        Ok(())
    }

    pub fn distribute_rewards(&self) -> Result<RewardDistribution, String> {
        let mut validators = self.validators.lock().map_err(|_| "Mutex lock failed")?;
        let mut rewards = HashMap::new();

        for (address, validator) in validators.iter_mut() {
            if validator.active {
                let reward = validator.locked_btcz / 1_000_000 * self.reward_rate;
                rewards.insert(address.clone(), reward);
                validator.last_reward_time = Utc::now();
            }
        }

        Ok(RewardDistribution {
            validator_rewards: rewards,
        })
    }

    pub fn slash_validator(&self, address: String, penalty: u64) -> Result<(), String> {
        let mut validators = self.validators.lock().map_err(|_| "Mutex lock failed")?;
        let validator = validators.get_mut(&address).ok_or("Validator not found")?;

        if validator.locked_btcz < penalty {
            return Err("Penalty exceeds locked BTCZ".to_string());
        }

        validator.locked_btcz -= penalty;
        if validator.locked_btcz == 0 {
            validator.active = false;
        }

        Ok(())
    }
}
