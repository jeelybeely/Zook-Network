// File: src/storage/keys.rs

use ed25519_dalek::{Keypair, PublicKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct KeyStore {
    keys: Arc<Mutex<HashMap<PublicKey, Keypair>>>,
}

impl KeyStore {
    /// Create a new, empty KeyStore.
    pub fn new() -> Self {
        KeyStore {
            keys: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Generate a new keypair and store it in the KeyStore.
    pub fn generate_keypair(&self) -> PublicKey {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public;
        self.keys.lock().unwrap().insert(public_key, keypair);
        public_key
    }

    /// Retrieve a keypair by public key.
    pub fn get_keypair(&self, public_key: &PublicKey) -> Option<Keypair> {
        self.keys.lock().unwrap().get(public_key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystore_operations() {
        let keystore = KeyStore::new();

        // Generate a keypair
        let public_key = keystore.generate_keypair();
        assert!(keystore.get_keypair(&public_key).is_some());

        // Retrieve the same keypair
        let retrieved_keypair = keystore.get_keypair(&public_key).unwrap();
        assert_eq!(retrieved_keypair.public, public_key);
    }
}
