
import os
import subprocess
import time
import requests

# Global configurations
BTCZ_REPO_URL = "https://github.com/btcz/bitcoinz.git"
ZOOK_REPO_URL = "https://github.com/zook-network/Zook-Network.git"
INSTALL_PATH = os.path.expanduser("~\\ZookBTCZTestnet")
BTCZ_CONF_PATH = os.path.join(INSTALL_PATH, "bitcoinz\\bitcoinz.conf")
ZOOK_ENV_PATH = os.path.join(INSTALL_PATH, "Zook-Network\\testnet.env")
CLARINET_TOML_PATH = os.path.join(INSTALL_PATH, "Zook-Network\\Clarinet.toml")
BTCZ_RPC_ENDPOINT = "http://127.0.0.1:8232"
RPC_USER = "zookrpcuser"
RPC_PASSWORD = "StrongPassword123"
STACKS_NETWORK = "testnet"

def run_command(command, cwd=None, check=True):
    try:
        subprocess.run(command, cwd=cwd, shell=True, check=check, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {command}")
        print(e.stderr.decode())
        raise

def clone_repositories():
    print("Cloning repositories...")
    zook_path = os.path.join(INSTALL_PATH, "Zook-Network")
    btcz_path = os.path.join(INSTALL_PATH, "bitcoinz")

    if not os.path.exists(zook_path):
        run_command(f"git clone {ZOOK_REPO_URL} {zook_path}")
    else:
        print("Zook repository already cloned.")

    if not os.path.exists(btcz_path):
        run_command(f"git clone {BTCZ_REPO_URL} {btcz_path}")
    else:
        print("BTCZ repository already cloned.")

def build_btcz_core():
    print("Building BTCZ Core...")
    btcz_path = os.path.join(INSTALL_PATH, "bitcoinz")
    msys_command = (
        "pacman -Syu --noconfirm && "
        "pacman -S base-devel mingw-w64-x86_64-toolchain automake libtool boost libevent --noconfirm && "
        "./autogen.sh && ./configure && make"
    )
    run_command(msys_command, cwd=btcz_path)
    print("BTCZ Core built successfully.")

def setup_btcz_configuration():
    print("Configuring BTCZ Core...")
    if not os.path.exists(BTCZ_CONF_PATH):
        os.makedirs(os.path.dirname(BTCZ_CONF_PATH), exist_ok=True)
        with open(BTCZ_CONF_PATH, "w") as conf_file:
            conf_file.write(f"""
testnet=1
rpcuser={RPC_USER}
rpcpassword={RPC_PASSWORD}
rpcallowip=127.0.0.1
server=1
txindex=1
""")
    print("BTCZ Core configured.")

def start_btcz_node():
    btcz_path = os.path.join(INSTALL_PATH, "bitcoinz")
    bitcoind_path = os.path.join(btcz_path, "src", "bitcoind.exe")
    print("Starting BTCZ Core node...")
    run_command(f"{bitcoind_path} -conf={BTCZ_CONF_PATH}")
    time.sleep(15)
    print("BTCZ Core node started.")

def deploy_clarity_contracts():
    print("Deploying Clarity contracts...")
    zook_path = os.path.join(INSTALL_PATH, "Zook-Network")
    run_command(f"clarinet deploy --network={STACKS_NETWORK}", cwd=zook_path)
    print("Clarity contracts deployed successfully.")

def start_zook_services():
    print("Starting Zook services...")
    zook_path = os.path.join(INSTALL_PATH, "Zook-Network")
    run_command("cargo run --release", cwd=zook_path)
    time.sleep(10)
    print("Zook services started.")

def run_end_to_end_tests():
    print("Running end-to-end tests...")
    payload = {
        "block_height": 1234,
        "state_root": "test_state_root",
        "merkle_proof": []
    }
    response = requests.post(f"{BTCZ_RPC_ENDPOINT}/sendanchor", json=payload, auth=(RPC_USER, RPC_PASSWORD))
    if response.status_code == 200:
        print("State anchoring RPC test passed.")
    else:
        raise Exception(f"State anchoring RPC test failed: {response.text}")

if __name__ == "__main__":
    try:
        if not os.path.exists(INSTALL_PATH):
            os.makedirs(INSTALL_PATH, exist_ok=True)
        clone_repositories()
        build_btcz_core()
        setup_btcz_configuration()
        start_btcz_node()
        deploy_clarity_contracts()
        start_zook_services()
        run_end_to_end_tests()
        print("Testnet setup completed successfully!")
    except Exception as e:
        print(f"Error: {str(e)}")
