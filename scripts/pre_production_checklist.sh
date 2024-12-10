# File: scripts/pre_production_checklist.sh

#!/bin/bash

# Configuration
API_URL="http://localhost:3030"

# Function to check an endpoint
check_endpoint() {
    local endpoint=$1
    echo "Checking endpoint: $endpoint"
    response=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL$endpoint")
    if [ "$response" -ne 200 ]; then
        echo "[ERROR] Endpoint $endpoint failed with status code $response"
        exit 1
    fi
    echo "[SUCCESS] Endpoint $endpoint is accessible"
}

# Function to benchmark an endpoint
benchmark_endpoint() {
    local endpoint=$1
    echo "Benchmarking endpoint: $endpoint"
    time curl -s "$API_URL$endpoint" > /dev/null
}

# Pre-Production Checklist

# 1. Validate Governance API
check_endpoint "/governance/parameters"
check_endpoint "/governance/propose"
check_endpoint "/governance/vote"
check_endpoint "/governance/execute"

# 2. Validate Bridge API
check_endpoint "/bridge/burn"
check_endpoint "/bridge/sync-event"
check_endpoint "/bridge/events"

# 3. Benchmark Governance API
benchmark_endpoint "/governance/parameters"

# 4. Benchmark Bridge API
benchmark_endpoint "/bridge/events"

echo "[SUCCESS] Pre-production checklist completed successfully."
