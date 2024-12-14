use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BridgeLedger {
    locked_balances: Arc<Mutex<HashMap<String, u64>>>,
}

impl BridgeLedger {
    pub fn new() -> Self {
        Self {
            locked_balances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn lock_btcz(&self, address: String, amount: u64) -> Result<(), String> {
        let mut balances = self.locked_balances.lock().await;
        let balance = balances.entry(address.clone()).or_insert(0);
        *balance += amount;
        Ok(())
    }

    pub async fn burn_zbtcz(
        &self,
        address: String,
        amount: u64,
        _transaction_hash: String,
    ) -> Result<(), String> {
        let mut balances = self.locked_balances.lock().await;
        let balance = balances.get_mut(&address).ok_or("Address not found")?;
        if *balance < amount {
            return Err("Insufficient balance".into());
        }
        *balance -= amount;
        Ok(())
    }
}
