// File: src/clarity/interaction.rs

use clarity_sdk::contract::Contract;
use clarity_sdk::client::Client;
use clarity_sdk::types::{Principal, Response};
use serde_json::json;

pub struct ClarityInteractor {
    client: Client,
    zbtcz_contract: Contract,
    governance_contract: Contract,
    bridge_contract: Contract,
}

impl ClarityInteractor {
    pub fn new(api_url: &str, sender: &Principal) -> Self {
        let client = Client::new(api_url.to_string(), sender.clone());

        let zbtcz_contract = Contract::new("zbtcz", sender.clone());
        let governance_contract = Contract::new("governance", sender.clone());
        let bridge_contract = Contract::new("bridge", sender.clone());

        Self {
            client,
            zbtcz_contract,
            governance_contract,
            bridge_contract,
        }
    }

    pub async fn mint_zbtcz(&self, amount: u128) -> Response {
        self.client.call(
            &self.zbtcz_contract,
            "mint-zbtcz",
            json!([amount]),
        ).await
    }

    pub async fn burn_zbtcz(&self, ids: Vec<u128>) -> Response {
        self.client.call(
            &self.zbtcz_contract,
            "burn-zbtcz",
            json!([ids]),
        ).await
    }

    pub async fn stake_btcz(&self, amount: u128) -> Response {
        self.client.call(
            &self.governance_contract,
            "stake-btcz",
            json!([amount]),
        ).await
    }

    pub async fn unstake_btcz(&self, amount: u128) -> Response {
        self.client.call(
            &self.governance_contract,
            "unstake-btcz",
            json!([amount]),
        ).await
    }

    pub async fn lock_btcz(&self, tx_id: u128, amount: u128) -> Response {
        self.client.call(
            &self.bridge_contract,
            "lock-btcz",
            json!([tx_id, amount]),
        ).await
    }

    pub async fn unlock_btcz(&self, tx_id: u128) -> Response {
        self.client.call(
            &self.bridge_contract,
            "unlock-btcz",
            json!([tx_id]),
        ).await
    }
}
