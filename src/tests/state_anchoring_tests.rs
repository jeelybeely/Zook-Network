// File: src/tests/state_anchoring_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::bridge::{BridgeModule, state_anchoring::L2StateSummary};
    use crate::bridge::validator::ValidatorState;
    use tokio::sync::Arc;

    #[tokio::test]
    async fn test_anchor_l2_state() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge = BridgeModule::new(vec![], validator_state.clone());

        let summary = bridge.anchor_l2_state(100, 50).unwrap();

        assert_eq!(summary.block_height, 100);
        assert_eq!(summary.total_transactions, 50);
        assert!(!summary.state_root.is_empty());

        println!("Anchored state: {:?}", summary);
    }

    #[tokio::test]
    async fn test_validate_l2_state() {
        let validator_state = Arc::new(ValidatorState::new());
        let bridge = BridgeModule::new(vec![], validator_state.clone());

        let summary = bridge.anchor_l2_state(200, 100).unwrap();
        let is_valid = bridge.validate_l2_state(&summary.state_root);

        assert!(is_valid);
        println!("State root validation passed for root: {}", summary.state_root);

        let is_invalid = bridge.validate_l2_state("invalid_root");
        assert!(!is_invalid);
        println!("State root validation failed for an invalid root.");
    }
}
