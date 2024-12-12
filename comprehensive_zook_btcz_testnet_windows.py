
import os
import subprocess
import time
import requests

# Global configurations
BTCZ_DOWNLOAD_URL = "https://github.com/btcz/bitcoinz/releases/download/v1.5.0/bitcoinz_win64.zip"
CLARINET_DOWNLOAD_URL = "https://github.com/clarity-lang/clarinet/releases/download/v1.0.2/clarinet-v1.0.2-win64.zip"
BTCZ_CONF_PATH = os.path.expanduser("~\\AppData\\Roaming\\BitcoinZ\\bitcoinz.conf")
INSTALL_PATH = os.path.expanduser("~\\ZookBTCZTestnet")
ZOOK_ENV_PATH = "./testnet.env"
CLARINET_TOML_PATH = "./Clarinet.toml"
BTCZ_RPC_ENDPOINT = "http://127.0.0.1:8232"
RPC_USER = "zookrpcuser"
RPC_PASSWORD = "StrongPassword123"
STACKS_NETWORK = "testnet"

def check_prerequisites():
    print("Checking prerequisites...")
    tools = ["cargo", "rustup"]
    for tool in tools:
        try:
            subprocess.run([tool, "--version"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
        except Exception:
            print(f"Missing prerequisite: {tool}. Please install it before proceeding.")
            exit(1)

def download_and_extract(url, extract_to):
    import zipfile
    response = requests.get(url, stream=True)
    zip_path = os.path.join(INSTALL_PATH, os.path.basename(url))
    with open(zip_path, "wb") as file:
        file.write(response.content)
    with zipfile.ZipFile(zip_path, "r") as zip_ref:
        zip_ref.extractall(extract_to)

def setup_btcz_core():
    print("Setting up BTCZ Core...")
    btcz_path = os.path.join(INSTALL_PATH, "bitcoinz")
    if not os.path.exists(btcz_path):
        os.makedirs(btcz_path, exist_ok=True)
        download_and_extract(BTCZ_DOWNLOAD_URL, btcz_path)
    
    if not os.path.exists(BTCZ_CONF_PATH):
        with open(BTCZ_CONF_PATH, "w") as conf_file:
            conf_file.write(f"""
testnet=1
rpcuser={RPC_USER}
rpcpassword={RPC_PASSWORD}
rpcallowip=127.0.0.1
server=1
txindex=1
""")
    subprocess.Popen([os.path.join(btcz_path, "bitcoind.exe"), "-conf", BTCZ_CONF_PATH])
    time.sleep(15)

def deploy_clarity_contracts():
    print("Deploying Clarity contracts...")
    clarinet_path = os.path.join(INSTALL_PATH, "clarinet")
    if not os.path.exists(clarinet_path):
        os.makedirs(clarinet_path, exist_ok=True)
        download_and_extract(CLARINET_DOWNLOAD_URL, clarinet_path)
    subprocess.run([os.path.join(clarinet_path, "clarinet.exe"), "deploy", f"--network={STACKS_NETWORK}"], check=True)

def start_zook_services():
    print("Starting Zook services...")
    if not os.path.exists(ZOOK_ENV_PATH):
        raise FileNotFoundError("The .env file for Zook Network is missing!")
    subprocess.Popen(["cargo", "run", "--release"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(10)

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
        check_prerequisites()
        setup_btcz_core()
        deploy_clarity_contracts()
        start_zook_services()
        run_end_to_end_tests()
        print("Testnet setup completed successfully!")
    except Exception as e:
        print(f"Error: {str(e)}")
