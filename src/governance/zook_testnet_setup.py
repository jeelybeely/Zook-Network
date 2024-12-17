import os
import subprocess
import shutil
import sys

def check_and_install_dependencies():
    """
    Checks for required dependencies and installs them if not found.
    """
    # Define the dependencies and their installation commands
    dependencies = {
        "clarinet": ["npm", "install", "-g", "@clarigen/core"]
    }

    # Path to npm
    npm_path = r"C:\node_modules\npm\bin\npm.cmd"  # Adjusted to your specific npm location

    # Check if npm exists
    if not os.path.isfile(npm_path):
        print(f"npm not found at {npm_path}. Ensure Node.js is correctly installed.")
        sys.exit(1)

    for dep, install_cmd in dependencies.items():
        if not shutil.which(dep):
            print(f"{dep} not found. Installing...")
            try:
                # Run the install command with npm
                subprocess.run([npm_path] + install_cmd[1:], check=True)
            except subprocess.CalledProcessError as e:
                print(f"Failed to install {dep}: {e}")
                sys.exit(1)

def clone_and_setup_zook():
    """
    Clones the Zook Network repository and sets it up for testing.
    """
    repo_url = "https://github.com/jeelybeely/Zook-Network"
    repo_dir = "Zook-Network"

    if not os.path.exists(repo_dir):
        print("Cloning Zook Network repository...")
        try:
            subprocess.run(["git", "clone", repo_url], check=True)
        except subprocess.CalledProcessError as e:
            print(f"Failed to clone repository: {e}")
            sys.exit(1)
    else:
        print("Zook Network repository already exists. Pulling latest changes...")
        try:
            subprocess.run(["git", "-C", repo_dir, "pull"], check=True)
        except subprocess.CalledProcessError as e:
            print(f"Failed to pull latest changes: {e}")
            sys.exit(1)

    # Set up the Zook Network testnet
    print("Setting up Zook Network testnet...")
    try:
        subprocess.run(["clarinet", "test"], cwd=repo_dir, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Failed to set up Zook testnet: {e}")
        sys.exit(1)

def main():
    """
    Main function to set up the Zook Network testnet environment.
    """
    print("Checking and installing dependencies...")
    check_and_install_dependencies()
    print("Dependencies installed successfully.")

    print("Cloning and setting up Zook Network...")
    clone_and_setup_zook()
    print("Zook Network testnet is ready.")

if __name__ == "__main__":
    main()
