#!/bin/bash

echo "Setting up Linera development environment..."

# Clone Linera repository
git clone https://github.com/linera-io/linera-protocol.git /workspace/linera-reference

# Build essential Linera components
cd /workspace/linera-reference
cargo build -p linera-sdk --features test

# Add Linera binaries to PATH
echo 'export PATH="/workspace/linera-reference/target/debug:$PATH"' >> ~/.bashrc

# Set up project structure
mkdir -p /workspace/linera-tictactoe/src
mkdir -p /workspace/linera-tictactoe/web

echo "Linera development environment setup complete!"
