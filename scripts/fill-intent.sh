#!/bin/bash
# Fill Intent Script (run as solver)

set -e

source .env 2>/dev/null || true

# Config
RPC_URL="${RPC_URL:-https://sepolia.base.org}"
GATEWAY="0xEC4c2DaEfeA63A15427c73976eaBAB945B76b463"
WETH="0x4200000000000000000000000000000000000006"

INTENT_ID="${1:-0}"
OUTPUT_AMOUNT="${2:-900000000000000}"  # Must be >= minOutputAmount

if [ -z "$SOLVER_PRIVATE_KEY" ]; then
    echo "❌ SOLVER_PRIVATE_KEY not set"
    exit 1
fi

echo "💰 Filling Intent #$INTENT_ID..."
echo "   Output Amount: $OUTPUT_AMOUNT WETH"

# Step 1: Check intent status
echo ""
echo "🔍 Checking intent status..."
INTENT=$(cast call $GATEWAY "getIntent(uint256)((address,address,uint256,address,uint256,uint256,uint8))" $INTENT_ID --rpc-url $RPC_URL)
echo "   Intent: $INTENT"

# Step 2: Approve WETH to gateway (solver needs WETH to pay creator)
echo ""
echo "📝 Approving WETH..."
cast send $WETH "approve(address,uint256)" $GATEWAY $OUTPUT_AMOUNT \
    --rpc-url $RPC_URL \
    --private-key $SOLVER_PRIVATE_KEY

echo "✅ Approved!"

# Step 3: Fill intent
echo ""
echo "📝 Filling intent..."
TX_HASH=$(cast send $GATEWAY "fillIntent(uint256,uint256)" $INTENT_ID $OUTPUT_AMOUNT \
    --rpc-url $RPC_URL \
    --private-key $SOLVER_PRIVATE_KEY \
    --json | jq -r '.transactionHash')

echo "✅ Intent Filled!"
echo "   TX Hash: $TX_HASH"

# Step 4: Verify
echo ""
echo "🔍 Verifying fill..."
INTENT_AFTER=$(cast call $GATEWAY "getIntent(uint256)((address,address,uint256,address,uint256,uint256,uint8))" $INTENT_ID --rpc-url $RPC_URL)
echo "   Intent After: $INTENT_AFTER"

echo ""
echo "🎉 Done! Intent #$INTENT_ID filled successfully!"
