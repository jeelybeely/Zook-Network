# File: tests/integration_rewards_test.py

import requests

# Configuration
ZOOK_NETWORK_URL = "http://localhost:3030"

# Helper functions
def distribute_rewards():
    print("Distributing rewards...")
    response = requests.post(f"{ZOOK_NETWORK_URL}/governance/distribute-rewards")
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to distribute rewards:", response.text)
        return False
    print("Rewards distributed successfully.")
    return True

def propose_reward_change(new_rate):
    print(f"Proposing reward rate change to {new_rate}...")
    proposal = {
        "description": "Change validator reward rate",
        "param": "reward-rate",
        "value": new_rate
    }
    response = requests.post(f"{ZOOK_NETWORK_URL}/governance/propose-reward-rate-change", json=proposal)
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to propose reward rate change:", response.text)
        return False
    print("Reward rate change proposed successfully.")
    return True

def execute_proposal(proposal_id):
    print(f"Executing proposal {proposal_id}...")
    response = requests.post(f"{ZOOK_NETWORK_URL}/governance/execute-proposal", json={"proposal_id": proposal_id})
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to execute proposal:", response.text)
        return False
    print("Proposal executed successfully.")
    return True

def test_validator_rewards():
    # Simulate rewards distribution
    if not distribute_rewards():
        print("Validator reward distribution test failed.")
        return False

    # Propose a reward rate change
    if not propose_reward_change(200):
        print("Reward rate change proposal test failed.")
        return False

    # Execute the proposal
    if not execute_proposal(1):  # Assuming proposal ID 1 for simplicity
        print("Proposal execution test failed.")
        return False

    print("All validator reward and governance tests passed successfully.")
    return True

if __name__ == "__main__":
    test_validator_rewards()