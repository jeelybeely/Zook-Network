use std::sync::Arc;
use crate::bridge::validator::ValidatorState;

#[derive(Debug, Clone)]
pub struct LockRequest {
    pub tx_id: String,
    pub amount: u64,
    pub from_address: String,
    pub to_address: String,
}

#[derive(Debug, Clone)]
pub struct BurnRequest {
    pub tx_id: String,
    pub amount: u64,
    pub from_address: String,
    pub to_address: String,
}

pub struct BridgeFinalization {
    validator_state: Arc<ValidatorState>,
}

impl BridgeFinalization {
    pub fn new(validator_state: Arc<ValidatorState>) -> Self {
        Self { validator_state }
    }

    pub fn lock_btc(&self, request: LockRequest) -> Result<(), String> {
        println!("Locking BTCZ: {:?}", request);
        if request.amount == 0 {
            return Err("Invalid lock amount".to_string());
        }

        // Correct use of validate_transaction
        if !self.validator_state.validate_transaction(&request.tx_id) {
            return Err("Transaction validation failed.".to_string());
        }

        println!("BTCZ successfully locked.");
        Ok(())
    }

    pub fn burn_btc(&self, request: BurnRequest) -> Result<(), String> {
        println!("Burning zBTCZ: {:?}", request);
        if request.amount == 0 {
            return Err("Invalid burn amount".to_string());
        }

        // Correct use of validate_transaction
        if !self.validator_state.validate_transaction(&request.tx_id) {
            return Err("Transaction validation failed.".to_string());
        }

        println!("zBTCZ successfully burned.");
        Ok(())
    }

    pub fn get_validator_state(&self) -> Arc<ValidatorState> {
        self.validator_state.clone()
    }
}
