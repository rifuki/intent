#!/bin/bash
# E2E Test Script for Intent System

# Base Sepolia Configuration
export RPC_URL="https://sepolia.base.org"
export GATEWAY_ADDRESS="0xEC4c2DaEfeA63A15427c73976eaBAB945B76b463"

# Base Sepolia Token Addresses
export WETH="0x4200000000000000000000000000000000000006"  # 18 decimals
export USDC="0x036CbD53842c5426634e7929541eC2318f3dCF7e"  # 6 decimals

# Load private key from .env (don't commit this!)
source .env 2>/dev/null || echo "No .env found, using PRIVATE_KEY env var"

echo "==================================="
echo "🧪 Intent System E2E Test"
echo "==================================="
echo "Gateway: $GATEWAY_ADDRESS"
echo "WETH: $WETH"
echo "USDC: $USDC"
echo ""

# Check if cast is installed
if ! command -v cast &> /dev/null; then
    echo "❌ 'cast' not found. Install foundry first: curl -L https://foundry.paradigm.xyz | bash"
    exit 1
fi

# Get wallet address from private key
WALLET=$(cast wallet address $PRIVATE_KEY)
echo "👛 Wallet: $WALLET"

# Check ETH balance
ETH_BALANCE=$(cast balance $WALLET --rpc-url $RPC_URL)
echo "💰 ETH Balance: $(cast from-wei $ETH_BALANCE) ETH"

# Check USDC balance
USDC_BALANCE=$(cast call $USDC "balanceOf(address)(uint256)" $WALLET --rpc-url $RPC_URL)
echo "💵 USDC Balance: $USDC_BALANCE (raw, 6 decimals)"

# Check WETH balance
WETH_BALANCE=$(cast call $WETH "balanceOf(address)(uint256)" $WALLET --rpc-url $RPC_URL)
echo "🪙 WETH Balance: $WETH_BALANCE (raw, 18 decimals)"

echo ""
echo "==================================="
echo "📋 Available Commands:"
echo "==================================="
echo ""
echo "1. Approve USDC to Gateway:"
echo "   cast send $USDC 'approve(address,uint256)' $GATEWAY_ADDRESS 1000000000 --rpc-url $RPC_URL --private-key \$PRIVATE_KEY"
echo ""
echo "2. Create Intent (USDC -> WETH):"
echo "   cast send $GATEWAY_ADDRESS 'createIntent(address,uint256,address,uint256,uint256)' \\"
echo "     $USDC 1000000 $WETH 900000000000000 \$((\$(date +%s) + 3600)) \\"
echo "     --rpc-url $RPC_URL --private-key \$PRIVATE_KEY"
echo ""
echo "3. Check Intent #0:"
echo "   cast call $GATEWAY_ADDRESS 'getIntent(uint256)((address,address,uint256,address,uint256,uint256,uint8))' 0 --rpc-url $RPC_URL"
echo ""
echo "4. Get Next Intent ID:"
echo "   cast call $GATEWAY_ADDRESS 'nextIntentId()(uint256)' --rpc-url $RPC_URL"
echo ""
