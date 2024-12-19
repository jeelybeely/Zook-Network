# Use a stable Rust version
FROM rust:1.83

# Set working directory
WORKDIR /zook

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    cmake \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Clone the Zook repository (always fetch latest)
RUN git clone https://github.com/jeelybeely/Zook-Network . \
    && git fetch origin \
    && git reset --hard origin/main

# Ensure dependencies are up to date
RUN cargo update

# Build the Zook project
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

# Expose ports for the Zook testnet
EXPOSE 20443 20444

# Run the Zook testnet by default
CMD ["cargo", "run", "--release", "--bin", "zook"]
