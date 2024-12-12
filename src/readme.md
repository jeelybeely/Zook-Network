# Source Code

The `src` directory contains the core implementation of the Zook Network, including APIs, authentication, bridging logic, governance workflows, and testing infrastructure. Each subdirectory focuses on a specific aspect of the network's functionality.

## **Subdirectories**

### 1. `api`
- **Purpose**: Defines REST API endpoints for interacting with the Zook Network.
- **Key Features**:
  - Exposes endpoints for bridging, governance, and validator rewards.
  - Facilitates external integrations with the Zook Network.

### 2. `auth`
- **Purpose**: Handles authentication and security mechanisms.
- **Key Features**:
  - Ensures secure access to the APIs.
  - Manages API keys and role-based access control.

### 3. `bridge`
- **Purpose**: Implements the bridging logic between BTCZ and the Zook Network.
- **Key Features**:
  - Processes BTCZ locking and `zBTCZ` minting.
  - Synchronizes burned `zBTCZ` with unlocked BTCZ.

### 4. `governance`
- **Purpose**: Manages on-chain governance mechanisms.
- **Key Features**:
  - Allows submission, voting, and execution of proposals.
  - Supports dynamic parameter adjustments through governance.

### 5. `rpc`
- **Purpose**: Provides RPC services for internal and external interactions.
- **Key Features**:
  - Facilitates state anchoring and cross-layer synchronization.
  - Supports advanced developer tooling for the network.

### 6. `scripts`
- **Purpose**: Contains helper scripts for internal operations.
- **Key Features**:
  - Automates maintenance tasks such as restarting services.
  - Simplifies network debugging and management.

### 7. `tests`
- **Purpose**: Comprehensive testing framework for the Zook Network.
- **Key Features**:
  - Includes unit tests for individual modules.
  - Provides integration and end-to-end tests for workflows.

### 8. `validator`
- **Purpose**: Manages validator registration, compliance, and rewards.
- **Key Features**:
  - Tracks validator performance and distributes rewards.
  - Logs compliance metrics for transparency.

## **How to Use**

1. **Building and Running**:
   - Use the provided scripts in `scripts` or compile directly using Rust.

2. **Testing**:
   - Run tests using `cargo test` or execute specific test cases in `src/tests`.

3. **Extending**:
   - Follow the modular design of the subdirectories to add new functionality.
   - Ensure compatibility with existing APIs, governance, and bridging logic.

## **Tips for Developers**
- Maintain separation of concerns by adhering to the existing modular structure.
- Document new modules thoroughly to aid future maintainers.
- Use integration tests in `src/tests` to validate changes across multiple modules.


