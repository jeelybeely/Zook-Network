// File: src/storage/storage.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub struct Wallet {
    balances: Arc<Mutex<HashMap<PublicKey, (u64, u64)>>>, // User balances: (zBTCZ, gBTCZ)
    keypairs: Arc<Mutex<HashMap<PublicKey, Keypair>>>,   // Mapping of public keys to keypairs
}

impl Wallet {
    /// Creates a new wallet storage.
    pub fn new() -> Self {
        Wallet {
            balances: Arc::new(Mutex::new(HashMap::new())),
            keypairs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Generates a new keypair and returns the public key.
    pub fn generate_keypair(&self) -> PublicKey {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public;
        self.keypairs.lock().unwrap().insert(public_key, keypair);
        public_key
    }

    /// Deposit tokens into a user's account.
    pub fn deposit(&self, public_key: &PublicKey, zbtcz: u64, gbtcz: u64) {
        let mut balances = self.balances.lock().unwrap();
        let entry = balances.entry(*public_key).or_insert((0, 0));
        entry.0 += zbtcz;
        entry.1 += gbtcz;
    }

    /// Withdraw tokens from a user's account, requiring a signed transaction.
    pub fn withdraw(
        &self,
        public_key: &PublicKey,
        zbtcz: u64,
        gbtcz: u64,
        signature: &Signature,
    ) -> Result<(), String> {
        let mut balances = self.balances.lock().unwrap();
        if let Some(entry) = balances.get_mut(public_key) {
            if entry.0 >= zbtcz && entry.1 >= gbtcz {
                // Verify the transaction signature
                let message = format!("WITHDRAW:{}:{}", zbtcz, gbtcz);
                let keypairs = self.keypairs.lock().unwrap();
                if let Some(keypair) = keypairs.get(public_key) {
                    if keypair.verify(message.as_bytes(), signature).is_ok() {
                        entry.0 -= zbtcz;
                        entry.1 -= gbtcz;
                        return Ok(());
                    } else {
                        return Err("Invalid signature".to_string());
                    }
                }
                Err("Keypair not found".to_string())
            } else {
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("User not found".to_string())
        }
    }

    /// Retrieve a user's balance.
    pub fn get_balance(&self, public_key: &PublicKey) -> Option<(u64, u64)> {
        let balances = self.balances.lock().unwrap();
        balances.get(public_key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_operations() {
        let wallet = Wallet::new();

        // Generate a keypair
        let public_key = wallet.generate_keypair();

        // Deposit tokens
        wallet.deposit(&public_key, 100, 200);
        assert_eq!(wallet.get_balance(&public_key), Some((100, 200)));

        // Create a signed withdrawal
        let keypairs = wallet.keypairs.lock().unwrap();
        let keypair = keypairs.get(&public_key).unwrap();
        let message = "WITHDRAW:50:100";
        let signature = keypair.sign(message.as_bytes());

        // Withdraw tokens
        assert!(wallet.withdraw(&public_key, 50, 100, &signature).is_ok());
        assert_eq!(wallet.get_balance(&public_key), Some((50, 100)));

        // Attempt to overdraw
        assert!(wallet.withdraw(&public_key, 100, 200, &signature).is_err());
    }
}
