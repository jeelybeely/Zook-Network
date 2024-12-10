# Zook Network Production Deployment Guide

## Overview

This guide provides the necessary steps to deploy the Zook Network to a production environment, including governance, bridge functionality, and cross-layer synchronization.

---

## Prerequisites

- **System Requirements**:
  - Minimum of 4 CPU cores and 16GB RAM for the server.
  - Stable internet connection for API services.
- **Dependencies**:
  - `clarinet` for contract deployment.
  - `curl` for testing and initialization.
- **Keys and Configuration**:
  - Replace placeholder deployer keys in the deployment scripts with actual production keys.

---

## Deployment Steps

### 1. Environment Setup

1. Install necessary dependencies:

   ```bash
   sudo apt update && sudo apt install -y curl build-essential
   cargo install clarinet-cli
   ```

2. Clone the Zook Network repository and navigate to the directory:

   ```bash
   git clone https://github.com/ZookNetwork/Zook.git
   cd Zook
   ```

3. Verify the server's readiness:

   ```bash
   ./scripts/pre_production_checklist.sh
   ```

---

### 2. Deploy Contracts

1. Run the deployment script:

   ```bash
   ./scripts/deploy.sh
   ```

2. Verify contract deployment by testing endpoints:

   ```bash
   curl -X GET http://localhost:3030/governance/parameters
   curl -X GET http://localhost:3030/bridge/events
   ```

---

### 3. Initialize Validators

1. Configure validators with the required parameters:

   ```bash
   curl -X POST http://localhost:3030/bridge/init -H "Content-Type: application/json" -d '{"merkle_root": "", "validators": ["validator1", "validator2"]}'
   ```

2. Verify validator initialization:

   ```bash
   curl -X GET http://localhost:3030/bridge/events
   ```

---

### 4. Monitoring Setup

1. Enable logging for all APIs:

   - Configure the logging level in `src/main.rs` or server configuration.

2. Use a monitoring tool like Prometheus for real-time insights:

   - Integrate logs with a monitoring stack for live alerting and analysis.

---

## Post-Deployment Checklist

- Validate governance proposals, voting, and execution:
  ```bash
  curl -X POST http://localhost:3030/governance/propose -H "Content-Type: application/json" -d '{"creator": "admin", "description": "Increase reward rate", "param": "reward-rate", "value": 200}'
  ```
- Test cross-layer event synchronization:
  ```bash
  curl -X POST http://localhost:3030/bridge/sync-event -H "Content-Type: application/json" -d '{"event_type": "burn", "tx_id": "tx123", "amount": 100, "merkle_root": "root123", "block_height": 500}'
  ```
- Monitor system performance under load using benchmark tools:
  ```bash
  ab -n 1000 -c 10 http://localhost:3030/bridge/events
  ```

---

## Troubleshooting

- **Contract Deployment Issues**:

  - Check Clarinet logs for errors.
  - Ensure the deployer keys and network settings are correct.

- **API Failures**:

  - Review server logs for detailed error messages.
  - Restart the API server if necessary.

- **Synchronization Delays**:

  - Verify the Merkle proofs and cross-layer event processing.

---

## Contact and Support

For assistance, contact the Zook Network team at hunterthnelson\@hotmail.com.

