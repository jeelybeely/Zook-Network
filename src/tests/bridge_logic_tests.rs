use crate::bridge::bridge_logic::BridgeLedger;

#[test]
fn test_lock_tokens() {
    let mut ledger = BridgeLedger::new();

    // Lock tokens
    assert!(ledger.lock_tokens("address1".to_string(), 100).is_ok());
    assert_eq!(ledger.get_locked_balance(&"address1".to_string()), 100);

    // Lock additional tokens
    assert!(ledger.lock_tokens("address1".to_string(), 50).is_ok());
    assert_eq!(ledger.get_locked_balance(&"address1".to_string()), 150);

    // Attempt to lock zero tokens
    assert!(ledger.lock_tokens("address1".to_string(), 0).is_err());
}

#[test]
fn test_burn_tokens() {
    let mut ledger = BridgeLedger::new();

    // Lock tokens first
    assert!(ledger.lock_tokens("address2".to_string(), 200).is_ok());

    // Burn tokens
    assert!(ledger.burn_tokens("address2".to_string(), 100).is_ok());
    assert_eq!(ledger.get_locked_balance(&"address2".to_string()), 100);
    assert_eq!(ledger.get_burned_balance(&"address2".to_string()), 100);

    // Attempt to burn more than locked
    assert!(ledger.burn_tokens("address2".to_string(), 150).is_err());

    // Attempt to burn zero tokens
    assert!(ledger.burn_tokens("address2".to_string(), 0).is_err());
}

#[test]
fn test_audit_token_flow() {
    let mut ledger = BridgeLedger::new();

    // Lock and burn tokens
    assert!(ledger.lock_tokens("address3".to_string(), 300).is_ok());
    assert!(ledger.burn_tokens("address3".to_string(), 100).is_ok());

    let (total_locked, total_burned) = ledger.audit_token_flow();
    assert_eq!(total_locked, 200);
    assert_eq!(total_burned, 100);
}
