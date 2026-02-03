#!/bin/bash
# Create Intent Script

set -e

source .env 2>/dev/null || true

# Config
RPC_URL="${RPC_URL:-https://sepolia.base.org}"
GATEWAY="0x4D7Ec71a5bD4Fcf7D56A3679518FDC55a8311683"
USDC="0x036CbD53842c5426634e7929541eC2318f3dCF7e"
WETH="0x4200000000000000000000000000000000000006"

# Params (adjust as needed)
INPUT_AMOUNT="${1:-1000000}"  # 1 USDC (6 decimals)
MIN_OUTPUT="${2:-900000000000000}"  # 0.0009 WETH (18 decimals)
DEADLINE=$(( $(date +%s) + 3600 ))  # 1 hour from now

echo "🔄 Creating Intent..."
echo "   Input: $INPUT_AMOUNT USDC"
echo "   Min Output: $MIN_OUTPUT WETH"
echo "   Deadline: $DEADLINE"

# Step 1: Approve USDC
echo ""
echo "📝 Step 1: Approving USDC..."
cast send $USDC "approve(address,uint256)" $GATEWAY $INPUT_AMOUNT \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY

echo "✅ Approved!"

# Step 2: Create Intent
echo ""
echo "📝 Step 2: Creating Intent..."
TX_HASH=$(cast send $GATEWAY "createIntent(address,uint256,address,uint256,uint256)" \
    $USDC $INPUT_AMOUNT $WETH $MIN_OUTPUT $DEADLINE \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --json | jq -r '.transactionHash')

echo "✅ Intent Created!"
echo "   TX Hash: $TX_HASH"

# Step 3: Get Intent ID
echo ""
echo "📋 Checking latest intent ID..."
NEXT_ID=$(cast call $GATEWAY "nextIntentId()(uint256)" --rpc-url $RPC_URL)
INTENT_ID=$(( NEXT_ID - 1 ))
echo "   Created Intent ID: $INTENT_ID"

# Step 4: Verify Intent
echo ""
echo "🔍 Verifying Intent #$INTENT_ID..."
cast call $GATEWAY "getIntent(uint256)((address,address,uint256,address,uint256,uint256,uint8))" $INTENT_ID --rpc-url $RPC_URL

echo ""
echo "🎉 Done! Intent #$INTENT_ID created successfully!"
echo ""
echo "Next steps:"
echo "  - Run solver: cd intent-solver && cargo run"
echo "  - Or fill manually: cast send $GATEWAY 'fillIntent(uint256,uint256)' $INTENT_ID $MIN_OUTPUT --rpc-url $RPC_URL --private-key \$SOLVER_KEY"
