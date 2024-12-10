// File: src/governance_scalability.rs

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub creator: String,
    pub description: String,
    pub param_change: Option<ParameterChange>,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterChange {
    pub param: String,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct GovernanceState {
    pub proposals: Arc<Mutex<Vec<Proposal>>>,
    pub next_proposal_id: Arc<Mutex<u64>>,
}

impl GovernanceState {
    pub fn new() -> Self {
        Self {
            proposals: Arc::new(Mutex::new(Vec::new())),
            next_proposal_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn create_proposal(&self, creator: String, description: String, param_change: Option<ParameterChange>) -> Result<u64, String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Mutex lock error")?;
        let mut next_id = self.next_proposal_id.lock().map_err(|_| "Mutex lock error")?;

        let proposal = Proposal {
            id: *next_id,
            creator,
            description,
            param_change,
            votes_for: 0,
            votes_against: 0,
            executed: false,
        };

        *next_id += 1;
        proposals.push(proposal);
        Ok(*next_id - 1)
    }

    pub fn vote_on_proposal(&self, proposal_id: u64, support: bool, voting_power: u64) -> Result<(), String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Mutex lock error")?;

        let proposal = proposals.iter_mut().find(|p| p.id == proposal_id).ok_or("Proposal not found")?;
        if proposal.executed {
            return Err("Proposal already executed".to_string());
        }

        if support {
            proposal.votes_for += voting_power;
        } else {
            proposal.votes_against += voting_power;
        }

        Ok(())
    }

    pub fn execute_proposal(&self, proposal_id: u64) -> Result<(), String> {
        let mut proposals = self.proposals.lock().map_err(|_| "Mutex lock error")?;

        let proposal = proposals.iter_mut().find(|p| p.id == proposal_id).ok_or("Proposal not found")?;
        if proposal.executed {
            return Err("Proposal already executed".to_string());
        }

        if proposal.votes_for > proposal.votes_against {
            proposal.executed = true;
            if let Some(change) = &proposal.param_change {
                println!("Executing parameter change: {} -> {}", change.param, change.value);
                // Apply parameter changes here
            }
            Ok(())
        } else {
            Err("Proposal did not pass".to_string())
        }
    }
}
