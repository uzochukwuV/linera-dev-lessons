#!/bin/bash

# Rock Paper Scissors Game Deployment Script

set -e

echo "🚀 Starting Rock Paper Scissors deployment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Build the contract and service WASM modules
echo -e "${BLUE}📦 Building WASM modules...${NC}"
cargo build --release --target wasm32-unknown-unknown

# Step 2: Initialize wallet with testnet faucet
echo -e "${BLUE}💳 Initializing wallet with testnet faucet...${NC}"
linera wallet init --with-new-chain --faucet https://faucet.testnet-babbage.linera.net || true

# Step 3: Get default chain ID
DEFAULT_CHAIN=$(linera wallet show | grep "Public Key" -A 1 | tail -n 1 | awk '{print $1}')
echo -e "${GREEN}✓ Default chain: ${DEFAULT_CHAIN}${NC}"

# Step 4: Publish the application bytecode
echo -e "${BLUE}📤 Publishing application bytecode...${NC}"
APP_ID=$(linera project publish-and-create \
  --required-application-ids [] \
  --json-parameters "{}" \
  --json-argument "{}" \
  2>&1 | grep -o 'e[0-9a-f]*' | head -n 1)

if [ -z "$APP_ID" ]; then
  echo -e "${YELLOW}⚠️  Failed to extract application ID. Trying alternative method...${NC}"

  # Alternative: publish and create separately
  BYTECODE_ID=$(linera project publish \
    --required-application-ids [] \
    2>&1 | grep -o 'e[0-9a-f]*' | head -n 1)

  echo -e "${GREEN}✓ Bytecode ID: ${BYTECODE_ID}${NC}"

  APP_ID=$(linera create-application \
    --bytecode-id "${BYTECODE_ID}" \
    --json-parameters "{}" \
    --json-argument "{}" \
    2>&1 | grep -o 'e[0-9a-f]*' | head -n 1)
fi

echo -e "${GREEN}✓ Application ID: ${APP_ID}${NC}"

# Step 5: Query the application
echo -e "${BLUE}🔍 Querying application state...${NC}"
linera query-application "${APP_ID}" "{ gameStatus }"

# Step 6: Start the Linera service
echo -e "${BLUE}🌐 Starting Linera service on port 8080...${NC}"
echo -e "${YELLOW}📝 Note: Save this Application ID for the UI: ${APP_ID}${NC}"
echo -e "${YELLOW}📝 Chain ID: ${DEFAULT_CHAIN}${NC}"
echo ""
echo -e "${GREEN}✅ Deployment complete!${NC}"
echo ""
echo -e "${BLUE}To use the application:${NC}"
echo -e "1. Save the Application ID: ${APP_ID}"
echo -e "2. Navigate to the webui directory: cd webui"
echo -e "3. Install dependencies: npm install"
echo -e "4. Build WASM module: npm run build:wasm"
echo -e "5. Start the UI: npm run dev"
echo -e "6. In the browser, store the Application ID in localStorage:"
echo -e "   localStorage.setItem('applicationId', '${APP_ID}')"
echo ""

# Start the service
linera service --port 8080
