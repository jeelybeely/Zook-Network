#!/bin/bash

set -e

# Runtime Deployment
echo "Starting Zook Runtime APIs..."

cargo run --release &

echo "Waiting for runtime to initialize..."
sleep 5

if curl -s http://127.0.0.1:3030/api/status > /dev/null; then
  echo "Runtime APIs are operational."
else
  echo "Failed to initialize runtime APIs."
  exit 1
fi

echo "Runtime deployment completed successfully."
