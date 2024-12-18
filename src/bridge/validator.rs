
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRecord {
    pub tx_id: String,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ValidatorState {
    pub processed_burns: Arc<Mutex<HashSet<String>>>,
}

impl ValidatorState {
    pub fn new() -> Self {
        Self {
            processed_burns: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn validate_burn(&self, record: &BurnRecord) -> Result<(), String> {
        let mut processed = self.processed_burns.lock().map_err(|_| "Mutex lock error")?;

        if processed.contains(&record.tx_id) {
            return Err("Duplicate burn transaction detected".to_string());
        }

        if record.amount == 0 {
            return Err("Invalid burn amount".to_string());
        }

        processed.insert(record.tx_id.clone());
        Ok(())
    }

    pub fn validate_transaction(&self, tx_id: &str) -> bool {
        // New method: validate if a transaction ID exists and meets criteria
        let processed = self.processed_burns.lock();
        match processed {
            Ok(processed) => processed.contains(tx_id),
            Err(_) => false,
        }
    }

    pub fn get_processed_burns(&self) -> Vec<String> {
        match self.processed_burns.lock() {
            Ok(processed) => processed.iter().cloned().collect(),
            Err(_) => Vec::new(), // Return an empty Vec if the lock fails
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_burn() {
        let validator = ValidatorState::new();
        let record = BurnRecord {
            tx_id: "tx123".to_string(),
            amount: 100,
            timestamp: 1640995200,
        };

        // Validate first burn
        assert!(validator.validate_burn(&record).is_ok());

        // Attempt duplicate burn
        assert!(validator.validate_burn(&record).is_err());

        // Attempt invalid amount
        let invalid_record = BurnRecord {
            tx_id: "tx124".to_string(),
            amount: 0,
            timestamp: 1640995200,
        };
        assert!(validator.validate_burn(&invalid_record).is_err());
    }

    #[test]
    fn test_validate_transaction() {
        let validator = ValidatorState::new();
        let tx_id = "tx123".to_string();

        assert_eq!(validator.validate_transaction(&tx_id), false);

        let record = BurnRecord {
            tx_id: tx_id.clone(),
            amount: 100,
            timestamp: 1640995200,
        };
        validator.validate_burn(&record).unwrap();

        assert_eq!(validator.validate_transaction(&tx_id), true);
    }
}



