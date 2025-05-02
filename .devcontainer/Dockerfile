FROM mcr.microsoft.com/devcontainers/rust:latest

# Install necessary dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    build-essential \
    curl \
    git \
    lld \
    clang \
    cmake \
    pkg-config \
    libssl-dev

# Install Wasm target
RUN rustup target add wasm32-unknown-unknown

# Install wasm-pack for frontend integration
RUN cargo install wasm-pack
