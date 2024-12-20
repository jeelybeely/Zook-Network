# Use a stable Rust version
FROM rust:1.83

# Set working directory
WORKDIR /zook

# Copy project files into the container
COPY . .

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Ensure dependencies are up to date
RUN cargo update

# Build the Zook project
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

# Expose port for the Zook server
EXPOSE 3030

# Run the server
CMD ["./target/release/zook-network"]
