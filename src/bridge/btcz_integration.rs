use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct BTCZAnchorPayload {
    pub block_height: u64,
    pub state_root: String,
    pub merkle_proof: Vec<String>,
    pub validator_compliance: bool,
}

pub struct BTCZIntegration {
    client: Client,
    btcz_endpoint: String,
}

impl BTCZIntegration {
    pub fn new(btcz_endpoint: String) -> Self {
        Self {
            client: Client::new(),
            btcz_endpoint,
        }
    }

    pub async fn send_anchor(
        &self,
        payload: BTCZAnchorPayload,
    ) -> Result<(), Box<dyn Error>> {
        let response = self
            .client
            .post(&self.btcz_endpoint)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            println!("Successfully anchored state to BTCZ: {:?}", payload);
            Ok(())
        } else {
            Err(format!(
                "Failed to anchor state: {}",
                response.text().await?
            )
            .into())
        }
    }
}
