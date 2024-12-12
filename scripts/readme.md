# Scripts

This directory contains various automation and helper scripts for deploying and managing the Zook Network. These scripts simplify tasks such as deploying smart contracts, interacting with APIs, and maintaining the network.

## **Key Scripts**

### 1. `deploy_contracts.sh`
- **Purpose**: Deploys Clarity smart contracts to the network.
- **Usage**:
  - Configure the contract deployment parameters in the script.
  - Run `./deploy_contracts.sh` to deploy.

### 2. `interact_with_api.py`
- **Purpose**: Provides an interface for interacting with the Zook Network's APIs.
- **Usage**:
  - Customize the API endpoints and payloads.
  - Run the script with Python: `python3 interact_with_api.py`.

### 3. `manage_network.sh`
- **Purpose**: Automates common network management tasks such as restarting nodes and cleaning logs.
- **Usage**:
  - Execute specific commands like `./manage_network.sh restart`.

## **How to Use**

1. Ensure all necessary dependencies are installed before running any scripts.
2. Follow the usage instructions within each script for proper execution.
3. For custom tasks, modify the scripts as needed but document any changes.

## **Tips for Developers**
- Always test scripts in a staging environment before using them in production.
- Maintain version control for scripts to track changes and updates.
- Use logging within scripts to capture and debug execution details.


