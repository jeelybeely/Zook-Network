// File: src/validator/node_registration.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorNode {
    pub address: String,
    pub registered_at: String,
    pub active: bool,
}

pub struct ValidatorRegistry {
    pub nodes: Arc<Mutex<HashMap<String, ValidatorNode>>>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_node(&self, address: String) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;

        if nodes.contains_key(&address) {
            return Err("Node already registered".to_string());
        }

        let node = ValidatorNode {
            address: address.clone(),
            registered_at: chrono::Utc::now().to_string(),
            active: true,
        };

        nodes.insert(address, node);
        Ok(())
    }

    pub fn list_nodes(&self) -> Result<Vec<ValidatorNode>, String> {
        let nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        Ok(nodes.values().cloned().collect())
    }

    pub fn deactivate_node(&self, address: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|_| "Mutex lock failed")?;
        if let Some(node) = nodes.get_mut(address) {
            node.active = false;
            Ok(())
        } else {
            Err("Node not found".to_string())
        }
    }
}
