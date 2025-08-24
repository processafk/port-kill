#!/bin/bash

# Installation script for port-kill
# This script builds and installs the port-kill application

set -e

echo "Building port-kill application..."

# Build the application
cargo build --release

echo "Build completed successfully!"

# Check if the binary was created
if [ -f "target/release/port-kill" ]; then
    echo "Binary created at: target/release/port-kill"
    echo ""
    echo "To run the application:"
    echo "  ./target/release/port-kill"
    echo ""
    echo "To run with logging:"
    echo "  RUST_LOG=info ./target/release/port-kill"
    echo ""
    echo "To test with sample servers:"
    echo "  ./test_ports.sh"
    echo "  RUST_LOG=info ./target/release/port-kill"
else
    echo "Error: Binary not found at target/release/port-kill"
    exit 1
fi
