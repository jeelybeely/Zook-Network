# Base image with Rust installed
FROM rust:latest

# Set working directory
WORKDIR /zook

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    cmake \
    libssl-dev

# Install Clarinet from source
WORKDIR /clarinet
RUN git clone https://github.com/hirosystems/clarinet.git . \
    && cargo build --release

# Add Clarinet to PATH
ENV PATH="/clarinet/target/release:$PATH"

# Clone the Zook repository
WORKDIR /zook
RUN git clone https://github.com/jeelybeely/Zook-Network . \
    && git pull

# Build the Zook project
RUN cargo build --release

# Expose ports for the Zook testnet
EXPOSE 20443 20444

# Run the Zook testnet by default
CMD ["cargo", "run", "--release", "--bin", "zook"]
