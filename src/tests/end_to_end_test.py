# File: tests/end_to_end_test.py

import requests

# Configuration
ZOOK_NETWORK_URL = "http://localhost:3030"

def test_contract_deployment():
    print("Testing contract deployment...")
    response = requests.get(f"{ZOOK_NETWORK_URL}/governance/parameters")
    if response.status_code != 200 or not response.json():
        print("Failed to verify contract deployment:", response.text)
        return False
    print("Contracts deployed and verified successfully.")
    return True

def test_bridge_initialization():
    print("Testing bridge initialization...")
    init_data = {
        "merkle_root": "initial_merkle_root",
        "validators": ["validator_1"]
    }
    response = requests.post(f"{ZOOK_NETWORK_URL}/bridge/init", json=init_data)
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to initialize bridge:", response.text)
        return False
    print("Bridge initialized successfully.")
    return True

def test_cross_layer_events():
    print("Testing cross-layer event synchronization...")
    event_data = {
        "event_type": "burn",
        "tx_id": "tx123",
        "amount": 100,
        "merkle_root": "root123",
        "block_height": 500
    }
    response = requests.post(f"{ZOOK_NETWORK_URL}/bridge/sync-event", json=event_data)
    if response.status_code != 200 or response.json().get("status") != "success":
        print("Failed to sync event:", response.text)
        return False

    events_response = requests.get(f"{ZOOK_NETWORK_URL}/bridge/events")
    if response.status_code != 200 or not any(e["tx_id"] == "tx123" for e in events_response.json()):
        print("Failed to verify synchronized event:", events_response.text)
        return False

    print("Cross-layer events synchronized successfully.")
    return True

def run_end_to_end_tests():
    if not test_contract_deployment():
        print("End-to-end test failed at contract deployment.")
        return

    if not test_bridge_initialization():
        print("End-to-end test failed at bridge initialization.")
        return

    if not test_cross_layer_events():
        print("End-to-end test failed at cross-layer event synchronization.")
        return

    print("All end-to-end tests passed successfully.")

if __name__ == "__main__":
    run_end_to_end_tests()
