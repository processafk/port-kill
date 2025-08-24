#!/bin/bash

# Port Kill - Easy Run Script
# This script runs the port-kill application with logging enabled

echo "🚀 Starting Port Kill..."
echo "📊 Status bar icon should appear shortly"
echo ""

# Run the application with logging
RUST_LOG=info ./target/release/port-kill
