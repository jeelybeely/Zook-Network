// File: tests/bridge_integration_tests.rs

use std::sync::Arc;
use tempfile::tempdir;
use crate::bridge::bridge_logic::BridgeLedger;
use crate::bridge::state_anchoring::StateAnchoring;
use crate::bridge::btcz_integration::BTCZIntegration;
use crate::bridge::merkle::MerkleTree;
use crate::clarity::ClarityContract;

#[tokio::test]
async fn test_lock_btcz_and_mint_zbtcz() {
    let temp_dir = tempdir().unwrap();
    let state_anchoring = StateAnchoring::default();
    let clarity_contract = Arc::new(ClarityContract::new(temp_dir.path().join("clarity.json")));
    let btcz_integration = Arc::new(BTCZIntegration::new("http://localhost:18332".to_string()));

    let mut bridge_ledger = BridgeLedger::new(state_anchoring, clarity_contract.clone(), btcz_integration.clone());

    let address = "test_address".to_string();
    let amount = 100;

    // Lock BTCZ and mint zBTCZ
    let result = bridge_ledger.lock_btcz(address.clone(), amount);
    assert!(result.is_ok(), "Failed to lock BTCZ: {:?}", result);

    // Check locked balance
    let locked_balance = bridge_ledger.get_locked_balance(&address);
    assert_eq!(locked_balance, amount, "Incorrect locked balance");
}

#[tokio::test]
async fn test_burn_zbtcz_and_unlock_btcz() {
    let temp_dir = tempdir().unwrap();
    let state_anchoring = StateAnchoring::default();
    let clarity_contract = Arc::new(ClarityContract::new(temp_dir.path().join("clarity.json")));
    let btcz_integration = Arc::new(BTCZIntegration::new("http://localhost:18332".to_string()));

    let mut bridge_ledger = BridgeLedger::new(state_anchoring, clarity_contract.clone(), btcz_integration.clone());

    let address = "test_address".to_string();
    let amount = 100;
    let transaction_hash = "test_transaction_hash".to_string();

    // Lock BTCZ and mint zBTCZ to prepare for burn
    bridge_ledger.lock_btcz(address.clone(), amount).expect("Failed to lock BTCZ");

    // Generate Merkle proof
    let merkle_tree = MerkleTree::new(vec![transaction_hash.clone()]);
    let proof = btcz_integration.generate_merkle_proof(&merkle_tree, &transaction_hash);
    assert!(btcz_integration.validate_merkle_proof(&merkle_tree, &transaction_hash, &proof), "Merkle proof validation failed");

    // Burn zBTCZ and unlock BTCZ
    let result = bridge_ledger.burn_zbtcz(address.clone(), amount, transaction_hash);
    assert!(result.is_ok(), "Failed to burn zBTCZ: {:?}", result);

    // Check burned balance
    let burned_balance = bridge_ledger.get_burned_balance(&address);
    assert_eq!(burned_balance, 0, "Incorrect burned balance");
}
