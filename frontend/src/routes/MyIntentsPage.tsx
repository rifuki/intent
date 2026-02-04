import { useAccount, useChainId, useReadContract, useWriteContract, useWaitForTransactionReceipt } from 'wagmi'
import { Clock, CheckCircle, XCircle, Loader2 } from 'lucide-react'

import { CONTRACTS } from '@/config/contracts'

// Gateway ABI
const gatewayAbi = [
    {
        name: 'nextIntentId',
        type: 'function',
        stateMutability: 'view',
        inputs: [],
        outputs: [{ name: '', type: 'uint256' }],
    },
    {
        name: 'getIntent',
        type: 'function',
        stateMutability: 'view',
        inputs: [{ name: 'intentId', type: 'uint256' }],
        outputs: [{
            name: '',
            type: 'tuple',
            components: [
                { name: 'creator', type: 'address' },
                { name: 'inputToken', type: 'address' },
                { name: 'inputAmount', type: 'uint256' },
                { name: 'outputToken', type: 'address' },
                { name: 'minOutputAmount', type: 'uint256' },
                { name: 'deadline', type: 'uint256' },
                { name: 'status', type: 'uint8' },
            ],
        }],
    },
    {
        name: 'cancelIntent',
        type: 'function',
        stateMutability: 'nonpayable',
        inputs: [{ name: 'intentId', type: 'uint256' }],
        outputs: [],
    },
] as const

type IntentStatus = 0 | 1 | 2

const statusConfig = {
    0: { label: 'Pending', icon: Clock, color: 'text-yellow-500' },
    1: { label: 'Filled', icon: CheckCircle, color: 'text-green-500' },
    2: { label: 'Cancelled', icon: XCircle, color: 'text-red-500' },
} as const

export function MyIntentsPage() {
    const { address, isConnected } = useAccount()
    const chainId = useChainId() as 84532 | 8453
    const contracts = CONTRACTS[chainId] || CONTRACTS[84532]

    // Get total intent count
    const { data: nextId, isLoading } = useReadContract({
        address: contracts.gateway,
        abi: gatewayAbi,
        functionName: 'nextIntentId',
    })

    if (!isConnected) {
        return (
            <div className="max-w-2xl mx-auto text-center py-16">
                <h1 className="text-2xl font-bold mb-4">My Intents</h1>
                <p className="text-muted-foreground">Please connect your wallet to view your intents.</p>
            </div>
        )
    }

    if (isLoading) {
        return (
            <div className="max-w-2xl mx-auto text-center py-16">
                <Loader2 className="h-8 w-8 animate-spin mx-auto mb-4" />
                <p className="text-muted-foreground">Loading intents...</p>
            </div>
        )
    }

    const intentCount = nextId ? Number(nextId) : 0

    return (
        <div className="max-w-2xl mx-auto">
            <h1 className="text-2xl font-bold mb-6">My Intents</h1>

            {intentCount === 0 ? (
                <div className="text-center py-12 border border-border rounded-lg">
                    <p className="text-muted-foreground">No intents found on this network.</p>
                </div>
            ) : (
                <div className="space-y-4">
                    {Array.from({ length: Math.min(intentCount, 10) }, (_, i) => (
                        <IntentCard
                            key={intentCount - 1 - i}
                            intentId={BigInt(intentCount - 1 - i)}
                            gatewayAddress={contracts.gateway}
                            userAddress={address!}
                        />
                    ))}
                </div>
            )}
        </div>
    )
}

function IntentCard({
    intentId,
    gatewayAddress,
    userAddress,
}: {
    intentId: bigint
    gatewayAddress: `0x${string}`
    userAddress: `0x${string}`
}) {
    const { data: intent, isLoading, refetch } = useReadContract({
        address: gatewayAddress,
        abi: gatewayAbi,
        functionName: 'getIntent',
        args: [intentId],
    })

    // Write contract hook for cancelling
    const { writeContract: cancelIntent, isPending: isCancelling, data: cancelHash } = useWriteContract()
    const { isSuccess: isCancelled } = useWaitForTransactionReceipt({
        hash: cancelHash
    })

    // Refetch when cancelled
    if (isCancelled) {
        refetch()
    }

    if (isLoading) {
        return (
            <div className="p-4 border border-border rounded-lg animate-pulse">
                <div className="h-4 bg-muted rounded w-1/4 mb-2"></div>
                <div className="h-3 bg-muted rounded w-1/2"></div>
            </div>
        )
    }

    if (!intent || intent.creator.toLowerCase() !== userAddress.toLowerCase()) {
        return null
    }

    const status = intent.status as IntentStatus
    const StatusIcon = statusConfig[status].icon
    const deadline = new Date(Number(intent.deadline) * 1000)
    const isExpired = deadline < new Date() && status === 0

    const handleCancel = () => {
        cancelIntent({
            address: gatewayAddress,
            abi: gatewayAbi,
            functionName: 'cancelIntent',
            args: [intentId],
        })
    }

    return (
        <div className="p-4 border border-border rounded-lg">
            <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-medium">Intent #{intentId.toString()}</span>
                <div className={`flex items-center gap-1 text-sm ${statusConfig[status].color}`}>
                    <StatusIcon className="h-4 w-4" />
                    {statusConfig[status].label}
                </div>
            </div>
            <div className="text-xs text-muted-foreground space-y-1 mb-3">
                <p>Input: {(Number(intent.inputAmount) / 1e6).toFixed(2)} (token)</p>
                <p>Min Output: {(Number(intent.minOutputAmount) / 1e18).toFixed(6)} (token)</p>
                <p>Deadline: {deadline.toLocaleString()}</p>
            </div>

            {status === 0 && (
                <button
                    onClick={handleCancel}
                    disabled={isCancelling}
                    className="w-full mt-2 px-3 py-2 bg-destructive/10 text-destructive text-sm rounded-md hover:bg-destructive/20 transition-colors flex items-center justify-center gap-2"
                >
                    {isCancelling ? (
                        <>
                            <Loader2 className="h-3 w-3 animate-spin" />
                            Cancelling...
                        </>
                    ) : (
                        'Cancel & Withdraw'
                    )}
                </button>
            )}
        </div>
    )
}
