// Contract addresses - properly typed
export const CONTRACTS = {
    // Base Sepolia (testnet)
    84532: {
        gateway: '0xEC4c2DaEfeA63A15427c73976eaBAB945B76b463' as `0x${string}`,
        usdc: '0x036CbD53842c5426634e7929541eC2318f3dCF7e' as `0x${string}`,
        weth: '0x4200000000000000000000000000000000000006' as `0x${string}`,
    },
    // Base Mainnet (placeholder - deploy and update)
    8453: {
        gateway: '0x0000000000000000000000000000000000000000' as `0x${string}`,
        usdc: '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913' as `0x${string}`,
        weth: '0x4200000000000000000000000000000000000006' as `0x${string}`,
    },
}

// Token info with proper address typing
export const TOKENS: Record<84532 | 8453, Array<{ address: `0x${string}`; symbol: string; decimals: number }>> = {
    84532: [
        { address: '0x036CbD53842c5426634e7929541eC2318f3dCF7e', symbol: 'USDC', decimals: 6 },
        { address: '0x4200000000000000000000000000000000000006', symbol: 'WETH', decimals: 18 },
    ],
    8453: [
        { address: '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913', symbol: 'USDC', decimals: 6 },
        { address: '0x4200000000000000000000000000000000000006', symbol: 'WETH', decimals: 18 },
    ],
}

// API base URL
export const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000'
