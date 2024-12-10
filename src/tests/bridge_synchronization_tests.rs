// File: src/tests/bridge_synchronization_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use tokio::sync::Arc;
    use crate::bridge::{BridgeModule, cross_layer_sync::{LockEvent, BurnEvent}};
    use crate::bridge::validator::ValidatorState;

    #[tokio::test]
    async fn test_record_lock_event() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge = BridgeModule::new(vec![], validator_state.clone());

        let lock_event = LockEvent {
            tx_id: "lock123".to_string(),
            amount: 100,
            from_address: "user1".to_string(),
            to_address: "bridge1".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let result = bridge.cross_layer_sync.record_lock_event(lock_event.clone());
        assert!(result.is_ok());

        let retrieved_event = bridge.cross_layer_sync.get_lock_event("lock123");
        assert!(retrieved_event.is_some());
        assert_eq!(retrieved_event.unwrap().tx_id, lock_event.tx_id);
    }

    #[tokio::test]
    async fn test_record_burn_event() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge = BridgeModule::new(vec![], validator_state.clone());

        let burn_event = BurnEvent {
            tx_id: "burn123".to_string(),
            amount: 50,
            from_address: "user2".to_string(),
            to_address: "bridge2".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let result = bridge.cross_layer_sync.record_burn_event(burn_event.clone());
        assert!(result.is_ok());

        let retrieved_event = bridge.cross_layer_sync.get_burn_event("burn123");
        assert!(retrieved_event.is_some());
        assert_eq!(retrieved_event.unwrap().tx_id, burn_event.tx_id);
    }

    #[tokio::test]
    async fn test_generate_merkle_proof() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge = BridgeModule::new(vec![], validator_state.clone());

        let lock_event = LockEvent {
            tx_id: "lock456".to_string(),
            amount: 200,
            from_address: "user3".to_string(),
            to_address: "bridge3".to_string(),
            timestamp: chrono::Utc::now(),
        };
        bridge.cross_layer_sync.record_lock_event(lock_event).unwrap();

        let proof = bridge.generate_proof("lock456", "lock");
        assert!(proof.is_ok());
        assert!(proof.unwrap().contains("Proof for tx_id lock456"));
    }
}
