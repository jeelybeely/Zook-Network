import os
import subprocess
import sys
import platform

def check_installation(command, install_instructions):
    """Check if a command is available, and print install instructions if not."""
    if subprocess.call(f"which {command}", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE) != 0:
        print(f"Error: {command} is not installed. {install_instructions}")
        sys.exit(1)

def install_dependencies():
    """Ensure all required tools are installed."""
    print("Checking for Python 3.x...")
    if not sys.version_info >= (3, 6):
        print("Error: Python 3.6 or higher is required.")
        sys.exit(1)

    print("Checking for Git...")
    check_installation("git", "Please install Git from https://git-scm.com.")

    print("Checking for Cargo (Rust)...")
    check_installation("cargo", "Please install Rust from https://rustup.rs.")

    print("Checking for npm (Node.js)...")
    check_installation("npm", "Please install Node.js and npm from https://nodejs.org.")

    print("Installing required Python packages...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "requests"])


def clone_repositories():
    """Clone required repositories."""
    repos = {
        "Zook-Network": "https://github.com/jeelybeely/Zook-Network.git",
        "bitcoinzL2": "https://github.com/jeelybeely/bitcoinzL2.git",
        "stacks-core": "https://github.com/stacks-network/stacks-core.git"
    }
    
    for repo_name, repo_url in repos.items():
        if not os.path.exists(repo_name):
            print(f"Cloning {repo_name}...")
            subprocess.check_call(["git", "clone", repo_url])
        else:
            print(f"{repo_name} already exists, pulling latest changes...")
            subprocess.check_call(["git", "-C", repo_name, "pull"])


def configure_zook():
    """Configure Zook Testnet."""
    print("Configuring Zook Testnet...")
    os.makedirs("Zook-Network/config", exist_ok=True)
    config_path = "Zook-Network/config/testnet.json"
    zook_config = {
        "rpc_port": 18332,
        "network": "testnet",
        "btcz_rpc": "http://127.0.0.1:18332",
        "stx_rpc": "http://127.0.0.1:20443"
    }

    with open(config_path, "w") as f:
        import json
        json.dump(zook_config, f, indent=4)


def configure_btcz():
    """Configure BitcoinZ Core for Testnet."""
    print("Configuring BTCZ Core for Testnet...")
    os.makedirs("bitcoinzL2/config", exist_ok=True)
    config_path = "bitcoinzL2/config/testnet.conf"
    btcz_config = """
testnet=1
txindex=1
rpcuser=testuser
rpcpassword=testpass
rpcport=18332
server=1
daemon=1
"""
    with open(config_path, "w") as f:
        f.write(btcz_config)


def configure_stx():
    """Configure Stacks Core for Testnet."""
    print("Configuring Stacks Core for Testnet...")
    os.makedirs("stacks-core/config", exist_ok=True)
    config_path = "stacks-core/config/testnet.toml"
    stx_config = """
[node]
rpc_bind = "0.0.0.0:20443"
bootstrap_node = true
miner = false
"""
    with open(config_path, "w") as f:
        f.write(stx_config)


def start_services():
    """Start Zook, BTCZ Core, and Stacks Core for Testnet."""
    print("Starting BitcoinZ Core...")
    subprocess.Popen(["bitcoind", "-conf=bitcoinzL2/config/testnet.conf"])

    print("Starting Stacks Core...")
    subprocess.Popen(["stacks-node", "start", "--config", "stacks-core/config/testnet.toml"])

    print("Starting Zook Network...")
    subprocess.Popen(["cargo", "run", "--manifest-path", "Zook-Network/Cargo.toml"])


def main():
    install_dependencies()
    clone_repositories()
    configure_zook()
    configure_btcz()
    configure_stx()
    start_services()
    print("Zook Testnet setup complete! Check logs for service status.")

if __name__ == "__main__":
    main()
