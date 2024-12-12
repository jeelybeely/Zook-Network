// File: src/storage/storage.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Storage {
    balances: Arc<Mutex<HashMap<String, (u64, u64)>>>, // User balances: (zBTCZ, gBTCZ)
}

impl Storage {
    /// Creates a new, empty storage.
    pub fn new() -> Self {
        Storage {
            balances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Deposit tokens into a user's account.
    pub fn deposit(&self, user_id: &str, zbtcz: u64, gbtcz: u64) {
        let mut balances = self.balances.lock().unwrap();
        let entry = balances.entry(user_id.to_string()).or_insert((0, 0));
        entry.0 += zbtcz;
        entry.1 += gbtcz;
    }

    /// Withdraw tokens from a user's account.
    pub fn withdraw(&self, user_id: &str, zbtcz: u64, gbtcz: u64) -> Result<(), String> {
        let mut balances = self.balances.lock().unwrap();
        if let Some(entry) = balances.get_mut(user_id) {
            if entry.0 >= zbtcz && entry.1 >= gbtcz {
                entry.0 -= zbtcz;
                entry.1 -= gbtcz;
                Ok(())
            } else {
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("User not found".to_string())
        }
    }

    /// Retrieve a user's balance.
    pub fn get_balance(&self, user_id: &str) -> Option<(u64, u64)> {
        let balances = self.balances.lock().unwrap();
        balances.get(user_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_operations() {
        let storage = Storage::new();

        // Deposit tokens
        storage.deposit("user1", 100, 200);
        assert_eq!(storage.get_balance("user1"), Some((100, 200)));

        // Withdraw tokens
        assert!(storage.withdraw("user1", 50, 100).is_ok());
        assert_eq!(storage.get_balance("user1"), Some((50, 100)));

        // Attempt to overdraw
        assert!(storage.withdraw("user1", 100, 200).is_err());
    }
}
