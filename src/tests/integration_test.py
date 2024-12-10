# File: tests/integration_test.py

import requests
import time

# Configuration
ZOOK_NETWORK_URL = "http://localhost:3030"
BTCZ_CORE_URL = "http://localhost:20443"

# Helper functions
def deploy_clarity_contracts():
    print("Deploying Clarity contracts...")
    # Assumes deploy_clarity.sh is properly configured and executable
    result = subprocess.run(["bash", "scripts/deploy_clarity.sh"], capture_output=True, text=True)
    if result.returncode != 0:
        print("Failed to deploy contracts:", result.stderr)
        return False
    print("Contracts deployed successfully.")
    return True

def test_minting_workflow():
    print("Testing minting workflow...")
    # Add finalized state to simulate BTCZ Core
    finalized_state = {
        "block_height": 100,
        "merkle_root": "abc123"
    }
    response = requests.post(f"{ZOOK_NETWORK_URL}/bridge/finalize", json=finalized_state)
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to add finalized state:", response.text)
        return False

    # Mint zBTCZ on L2
    mint_request = {
        "amount": 10,
        "block_height": 100,
        "merkle_root": "abc123"
    }
    response = requests.post(f"{ZOOK_NETWORK_URL}/mint-zbtcz", json=mint_request)
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to mint zBTCZ:", response.text)
        return False

    print("Minting workflow successful.")
    return True

def test_burning_workflow():
    print("Testing burning workflow...")
    # Simulate burning zBTCZ on L2
    burn_request = {
        "ids": [1, 2, 3]
    }
    response = requests.post(f"{ZOOK_NETWORK_URL}/burn-zbtcz", json=burn_request)
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to burn zBTCZ:", response.text)
        return False

    # Sync burn records to L1
    burn_sync_data = [{"tx-id": "tx123", "amount": 3}]
    response = requests.post(f"{BTCZ_CORE_URL}/bridge/burn-sync", json=burn_sync_data)
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to sync burn records:", response.text)
        return False

    print("Burning workflow successful.")
    return True

def run_integration_tests():
    if not deploy_clarity_contracts():
        print("Failed to deploy contracts. Aborting tests.")
        return

    print("Running integration tests...")
    if not test_minting_workflow():
        print("Minting workflow test failed.")
        return

    if not test_burning_workflow():
        print("Burning workflow test failed.")
        return

    print("All integration tests passed successfully.")

if __name__ == "__main__":
    run_integration_tests()
