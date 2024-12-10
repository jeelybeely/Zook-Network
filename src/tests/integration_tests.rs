// File: src/tests/integration_tests.rs

#[cfg(test)]
mod integration_tests {
    use super::*;
    use clarity_sdk::client::Client;
    use clarity_sdk::types::{Principal, Response};
    use tokio::sync::Arc;
    use crate::clarity::interaction::ClarityInteractor;

    #[tokio::test]
    async fn test_mint_and_burn_zbtcz() {
        let clarity = Arc::new(ClarityInteractor::new("http://localhost:20443", &"ST123".into()));

        // Mint zBTCZ
        let mint_response = clarity.mint_zbtcz(1000).await;
        assert!(mint_response.is_ok(), "Minting failed: {:?}", mint_response);
        println!("Mint Response: {:?}", mint_response);

        // Burn zBTCZ
        let burn_response = clarity.burn_zbtcz(vec![1, 2, 3]).await;
        assert!(burn_response.is_ok(), "Burning failed: {:?}", burn_response);
        println!("Burn Response: {:?}", burn_response);
    }

    #[tokio::test]
    async fn test_lock_and_unlock_btcz() {
        let clarity = Arc::new(ClarityInteractor::new("http://localhost:20443", &"ST123".into()));

        // Lock BTCZ
        let lock_response = clarity.lock_btcz(12345, 1000).await;
        assert!(lock_response.is_ok(), "Locking failed: {:?}", lock_response);
        println!("Lock Response: {:?}", lock_response);

        // Unlock BTCZ
        let unlock_response = clarity.unlock_btcz(12345).await;
        assert!(unlock_response.is_ok(), "Unlocking failed: {:?}", unlock_response);
        println!("Unlock Response: {:?}", unlock_response);
    }
}
