import os
import subprocess
import shutil

def check_and_install_dependencies():
    dependencies = ["git", "cargo", "clarinet"]

    for dep in dependencies:
        if not shutil.which(dep):
            print(f"{dep} not found. Installing...")
            if dep == "git":
                subprocess.run(["winget", "install", "--id", "Git.Git"], check=True)
            elif dep == "cargo":
                subprocess.run(["winget", "install", "--id", "Rustlang.Rustup"], check=True)
                subprocess.run(["rustup", "install", "stable"], check=True)
            elif dep == "clarinet":
                subprocess.run(["npm", "install", "-g", "@clarigen/core"], check=True)


def clone_or_update_repository(repo_url, target_dir):
    if not os.path.exists(target_dir):
        print(f"Cloning repository from {repo_url}...")
        subprocess.run(["git", "clone", repo_url, target_dir], check=True)
    else:
        print(f"Repository already exists at {target_dir}. Pulling latest changes...")
        subprocess.run(["git", "-C", target_dir, "pull"], check=True)


def setup_environment_variables(env_file):
    if os.path.exists(env_file):
        print(f"Loading environment variables from {env_file}...")
        with open(env_file, "r") as f:
            for line in f:
                if line.strip() and not line.startswith("#"):
                    key, value = line.strip().split("=", 1)
                    os.environ[key] = value


def run_testnet(project_dir):
    print("Building the project...")
    subprocess.run(["cargo", "build"], cwd=project_dir, check=True)

    print("Starting the Zook testnet...")
    subprocess.run(["cargo", "run", "--", "--testnet"], cwd=project_dir, check=True)


def run_tests(project_dir):
    print("Running test suites...")
    subprocess.run(["cargo", "test"], cwd=project_dir, check=True)


def main():
    repo_url = "https://github.com/jeelybeely/Zook-Network"
    target_dir = "./Zook-Network"
    env_file = os.path.join(target_dir, "testnet.env")

    print("Checking and installing dependencies...")
    check_and_install_dependencies()

    print("Cloning or updating repository...")
    clone_or_update_repository(repo_url, target_dir)

    print("Setting up environment variables...")
    setup_environment_variables(env_file)

    print("Running the Zook testnet...")
    run_testnet(target_dir)

    print("Running tests...")
    run_tests(target_dir)

if __name__ == "__main__":
    main()
