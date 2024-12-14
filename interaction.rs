// File: interaction.rs

use serde_json::json;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Principal(String);

impl Principal {
    pub fn new(address: String) -> Self {
        Principal(address)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>,
}

/// Interactor for Clarity contract interactions
#[derive(Clone, Debug)]
pub struct ClarityInteractor {
    api_url: String,
    sender: Principal,
    zbtcz_address: String,
    gbtcz_address: String,
    governance_address: String,
    client: HttpClient,
}

impl ClarityInteractor {
    pub fn new(
        api_url: &str,
        sender: Principal,
        zbtcz_address: &str,
        gbtcz_address: &str,
        governance_address: &str,
    ) -> Self {
        Self {
            api_url: api_url.to_string(),
            sender,
            zbtcz_address: zbtcz_address.to_string(),
            gbtcz_address: gbtcz_address.to_string(),
            governance_address: governance_address.to_string(),
            client: HttpClient::new(),
        }
    }

    pub async fn mint_zbtcz(&self, amount: u128) -> Response {
        self.call_contract(&self.zbtcz_address, "mint-zbtcz", json!([amount])).await
    }

    pub async fn burn_zbtcz(&self, ids: Vec<u128>) -> Response {
        self.call_contract(&self.zbtcz_address, "burn-zbtcz", json!([ids])).await
    }

    pub async fn stake_gbtcz(&self, amount: u128) -> Response {
        self.call_contract(&self.gbtcz_address, "stake-gbtcz", json!([amount])).await
    }

    pub async fn unstake_gbtcz(&self, amount: u128) -> Response {
        self.call_contract(&self.gbtcz_address, "unstake-gbtcz", json!([amount])).await
    }

    pub async fn lock_btcz(&self, tx_id: u128, amount: u128) -> Response {
        self.call_contract(&self.governance_address, "lock-btcz", json!([tx_id, amount])).await
    }

    pub async fn unlock_btcz(&self, tx_id: u128) -> Response {
        self.call_contract(&self.governance_address, "unlock-btcz", json!([tx_id])).await
    }

    async fn call_contract(
        &self,
        contract_address: &str,
        method: &str,
        args: Value,
    ) -> Response {
        let endpoint = format!("{}/v2/contracts/call-read/{}/{}", self.api_url, contract_address, method);
        let payload = json!({
            "sender": self.sender.0,
            "arguments": args,
        });

        match self.client.post(&endpoint).json(&payload).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data = resp.json::<Value>().await.unwrap_or_default();
                    Response {
                        success: true,
                        message: "Contract call successful".to_string(),
                        data: Some(data),
                    }
                } else {
                    Response {
                        success: false,
                        message: format!("Error: {}", resp.status()),
                        data: None,
                    }
                }
            }
            Err(err) => Response {
                success: false,
                message: format!("HTTP request failed: {}", err),
                data: None,
            },
        }
    }
}
