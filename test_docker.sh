#!/bin/bash

# Docker Integration Test Script
# This script tests the Docker integration by starting test containers and monitoring them

echo "🐳 Testing Docker Integration..."
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker Desktop and try again."
    exit 1
fi

echo "✅ Docker is running"

# Stop any existing test containers
echo "🧹 Cleaning up existing test containers..."
docker stop test-react-3000 test-react-3001 test-node-8000 2>/dev/null || true
docker rm test-react-3000 test-react-3001 test-node-8000 2>/dev/null || true

# Start test containers
echo "🚀 Starting test containers..."

# Start React app on port 3000
echo "   Starting React app on port 3000..."
docker run -d --name test-react-3000 -p 3000:3000 nginx:alpine &
sleep 2

# Start React app on port 3001
echo "   Starting React app on port 3001..."
docker run -d --name test-react-3001 -p 3001:3000 nginx:alpine &
sleep 2

# Start Node.js app on port 8000
echo "   Starting Node.js app on port 8000..."
docker run -d --name test-node-8000 -p 8000:3000 nginx:alpine &
sleep 2

# Wait for containers to start
echo "⏳ Waiting for containers to start..."
sleep 5

# Check if containers are running
echo "📋 Checking container status..."
docker ps --filter "name=test-" --format "table {{.Names}}\t{{.Ports}}\t{{.Status}}"

echo ""
echo "🔍 Now testing port-kill with Docker monitoring..."
echo "   Press Ctrl+C to stop the test"
echo ""

# Run port-kill with Docker monitoring
RUST_LOG=info ./target/release/port-kill --console --docker --ports 3000,3001,8000 --verbose
