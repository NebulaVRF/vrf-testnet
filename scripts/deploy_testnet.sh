#!/usr/bin/env bash
set -euo pipefail

# Deploy the NebulaVRF contract to Soroban testnet.
#
# Usage:
#   SOURCE_ACCOUNT=<identity> [ALIAS=<alias>] [CONTRACT_DIR=contracts/nebula-vrf] ./scripts/deploy_testnet.sh
#
# Example:
#   SOURCE_ACCOUNT=nebula-vrf-deployer ALIAS=nebula-vrf ./scripts/deploy_testnet.sh

CONTRACT_DIR="${CONTRACT_DIR:-contracts/nebula-vrf}"
SOURCE_ACCOUNT="${SOURCE_ACCOUNT:-}"
ALIAS="${ALIAS:-}"
NETWORK="${NETWORK:-testnet}"

if [[ -z "$SOURCE_ACCOUNT" ]]; then
  echo "ERROR: SOURCE_ACCOUNT is required (stellar identity name)." >&2
  exit 1
fi

if [[ ! -d "$CONTRACT_DIR" ]]; then
  echo "ERROR: CONTRACT_DIR not found: $CONTRACT_DIR" >&2
  exit 1
fi

echo "Building contract in $CONTRACT_DIR..."
(cd "$CONTRACT_DIR" && stellar contract build)

WASM_PATH="$CONTRACT_DIR/target/wasm32v1-none/release/nebula_vrf.wasm"
if [[ ! -f "$WASM_PATH" ]]; then
  echo "ERROR: WASM not found at $WASM_PATH" >&2
  exit 1
fi

echo "Deploying to $NETWORK as $SOURCE_ACCOUNT..."

if [[ -n "$ALIAS" ]]; then
  stellar contract deploy \
    --wasm "$WASM_PATH" \
    --source-account "$SOURCE_ACCOUNT" \
    --network "$NETWORK" \
    --alias "$ALIAS"
else
  stellar contract deploy \
    --wasm "$WASM_PATH" \
    --source-account "$SOURCE_ACCOUNT" \
    --network "$NETWORK"
fi
