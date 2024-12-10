// File: src/governance/mod.rs

pub mod governance_scalability;

use governance_scalability::{GovernanceState, ParameterChange};
use std::sync::Arc;

pub struct GovernanceModule {
    pub state: Arc<GovernanceState>,
}

impl GovernanceModule {
    pub fn new() -> Self {
        Self {
            state: Arc::new(GovernanceState::new()),
        }
    }

    pub fn propose_parameter_change(&self, creator: String, description: String, param: String, value: u64) -> Result<u64, String> {
        let param_change = Some(ParameterChange { param, value });
        self.state.create_proposal(creator, description, param_change)
    }

    pub fn vote(&self, proposal_id: u64, support: bool, voting_power: u64) -> Result<(), String> {
        self.state.vote_on_proposal(proposal_id, support, voting_power)
    }

    pub fn execute(&self, proposal_id: u64) -> Result<(), String> {
        self.state.execute_proposal(proposal_id)
    }
}
