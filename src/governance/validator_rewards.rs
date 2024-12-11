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
pub struct ValidatorComplianceLog {
    pub timestamp: DateTime<Utc>,
    pub is_compliant: bool,
    pub compliance_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub validator_rewards: HashMap<String, u64>, // Validator address to reward amount
}

pub struct ValidatorRewards {
    pub validators: Arc<Mutex<HashMap<String, Validator>>>,
    pub compliance_logs: Arc<Mutex<HashMap<String, Vec<ValidatorComplianceLog>>>>, // Compliance logs by validator
    pub reward_rate: u64, // Reward per block in gBTCZ
}

impl ValidatorRewards {
    pub fn new(reward_rate: u64) -> Self {
        Self {
            validators: Arc::new(Mutex::new(HashMap::new())),
            compliance_logs: Arc::new(Mutex::new(HashMap::new())),
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

        validators.insert(address.clone(), validator);
        self.compliance_logs.lock().map_err(|_| "Mutex lock failed")?.insert(address, Vec::new());
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

        self.log_compliance(
            address.clone(),
            false,
            format!("Slashed for penalty: {}", penalty),
        );

        Ok(())
    }

    pub fn log_compliance(&self, address: String, is_compliant: bool, reason: String) {
        let log = ValidatorComplianceLog {
            timestamp: Utc::now(),
            is_compliant,
            compliance_reason: reason,
        };

        self.compliance_logs
            .lock()
            .map_err(|_| "Mutex lock failed")
            .unwrap()
            .entry(address)
            .or_insert_with(Vec::new)
            .push(log);
    }

    pub fn get_compliance_logs(
        &self,
        address: &String,
    ) -> Option<Vec<ValidatorComplianceLog>> {
        self.compliance_logs
            .lock()
            .ok()
            .and_then(|logs| logs.get(address).cloned())
    }

    pub fn get_latest_compliance(
        &self,
        address: &String,
    ) -> Option<ValidatorComplianceLog> {
        self.compliance_logs
            .lock()
            .ok()
            .and_then(|logs| logs.get(address).and_then(|l| l.last().cloned()))
    }
}
