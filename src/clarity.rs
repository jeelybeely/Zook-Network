

pub use crate::interaction::{ClarityInteractor, Principal, Response}; // Adjusted path to re-export the required structs.

//use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Clarity {
    interactor: Arc<ClarityInteractor>,
}

impl Clarity {
    pub fn new(
        api_url: &str,
        sender: Principal,
        zbtcz_address: &str,
        gbtcz_address: &str,
        governance_address: &str,
    ) -> Self {
        let interactor = ClarityInteractor::new(
            api_url,
            sender,
            zbtcz_address,
            gbtcz_address,
            governance_address,
        );
        Self {
            interactor: Arc::new(interactor),
        }
    }

    pub async fn mint_zbtcz(&self, amount: u128) -> Response {
        self.interactor.mint_zbtcz(amount).await
    }

    pub async fn burn_zbtcz(&self, ids: Vec<u128>) -> Response {
        self.interactor.burn_zbtcz(ids).await
    }

    pub async fn stake_gbtcz(&self, amount: u128) -> Response {
        self.interactor.stake_gbtcz(amount).await
    }

    pub async fn unstake_gbtcz(&self, amount: u128) -> Response {
        self.interactor.unstake_gbtcz(amount).await
    }

    pub async fn lock_btcz(&self, tx_id: u128, amount: u128) -> Response {
        self.interactor.lock_btcz(tx_id, amount).await
    }

    pub async fn unlock_btcz(&self, tx_id: u128) -> Response {
        self.interactor.unlock_btcz(tx_id).await
    }
}
