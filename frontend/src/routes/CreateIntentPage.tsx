import { useState } from 'react'
import { useAccount, useChainId, useWriteContract, useWaitForTransactionReceipt } from 'wagmi'
import { parseUnits } from 'viem'
import { Loader2, ArrowDown, CheckCircle } from 'lucide-react'

import { CONTRACTS, TOKENS } from '@/config/contracts'

// IntentGateway ABI (only what we need)
const gatewayAbi = [
    {
        name: 'createIntent',
        type: 'function',
        stateMutability: 'nonpayable',
        inputs: [
            { name: 'inputToken', type: 'address' },
            { name: 'inputAmount', type: 'uint256' },
            { name: 'outputToken', type: 'address' },
            { name: 'minOutputAmount', type: 'uint256' },
            { name: 'deadline', type: 'uint256' },
        ],
        outputs: [{ name: 'intentId', type: 'uint256' }],
    },
] as const

const erc20Abi = [
    {
        name: 'approve',
        type: 'function',
        stateMutability: 'nonpayable',
        inputs: [
            { name: 'spender', type: 'address' },
            { name: 'amount', type: 'uint256' },
        ],
        outputs: [{ name: '', type: 'bool' }],
    },
] as const

export function CreateIntentPage() {
    const { isConnected } = useAccount()
    const chainId = useChainId() as 84532 | 8453

    const [inputToken, setInputToken] = useState(0) // index
    const [outputToken, setOutputToken] = useState(1) // index
    const [inputAmount, setInputAmount] = useState('')
    const [minOutput, setMinOutput] = useState('')
    const [deadline, setDeadline] = useState('1') // hours
    const [step, setStep] = useState<'form' | 'approve' | 'create' | 'done'>('form')

    const tokens = TOKENS[chainId] || TOKENS[84532]
    const contracts = CONTRACTS[chainId] || CONTRACTS[84532]

    const { writeContract: approveWrite, data: approveHash } = useWriteContract()
    const { writeContract: createWrite, data: createHash } = useWriteContract()

    const { isLoading: isApproving } = useWaitForTransactionReceipt({ hash: approveHash })
    const { isLoading: isCreating, isSuccess: isCreated } = useWaitForTransactionReceipt({ hash: createHash })

    const handleApprove = () => {
        if (!inputAmount) return

        const token = tokens[inputToken]
        const amount = parseUnits(inputAmount, token.decimals)

        setStep('approve')
        approveWrite({
            address: token.address,
            abi: erc20Abi,
            functionName: 'approve',
            args: [contracts.gateway, amount],
        })
    }

    const handleCreate = () => {
        if (!inputAmount || !minOutput) return

        const inToken = tokens[inputToken]
        const outToken = tokens[outputToken]
        const inAmount = parseUnits(inputAmount, inToken.decimals)
        const outAmount = parseUnits(minOutput, outToken.decimals)
        const deadlineTimestamp = BigInt(Math.floor(Date.now() / 1000) + parseInt(deadline) * 3600)

        setStep('create')
        createWrite({
            address: contracts.gateway,
            abi: gatewayAbi,
            functionName: 'createIntent',
            args: [inToken.address, inAmount, outToken.address, outAmount, deadlineTimestamp],
        })
    }

    if (!isConnected) {
        return (
            <div className="max-w-lg mx-auto text-center py-16">
                <h1 className="text-2xl font-bold mb-4">Create Intent</h1>
                <p className="text-muted-foreground">Please connect your wallet to create an intent.</p>
            </div>
        )
    }

    if (isCreated || step === 'done') {
        return (
            <div className="max-w-lg mx-auto text-center py-16">
                <CheckCircle className="h-16 w-16 text-green-500 mx-auto mb-4" />
                <h1 className="text-2xl font-bold mb-2">Intent Created!</h1>
                <p className="text-muted-foreground mb-4">
                    Your intent has been submitted. Solvers will compete to fill it.
                </p>
                {createHash && (
                    <a
                        href={`https://sepolia.basescan.org/tx/${createHash}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-sm text-primary hover:underline"
                    >
                        View on Basescan →
                    </a>
                )}
                <button
                    onClick={() => {
                        setStep('form')
                        setInputAmount('')
                        setMinOutput('')
                    }}
                    className="block mx-auto mt-6 px-4 py-2 border border-border rounded-lg text-sm hover:bg-accent transition-colors"
                >
                    Create Another
                </button>
            </div>
        )
    }

    return (
        <div className="max-w-lg mx-auto">
            <h1 className="text-2xl font-bold mb-6">Create Intent</h1>

            <div className="border border-border rounded-lg p-6 space-y-6">
                {/* Input Token */}
                <div>
                    <label className="block text-sm font-medium mb-2">You Send</label>
                    <div className="flex gap-2">
                        <select
                            value={inputToken}
                            onChange={(e) => setInputToken(Number(e.target.value))}
                            className="flex-1 px-3 py-2 bg-secondary border border-border rounded-lg text-sm"
                        >
                            {tokens.map((t, i) => (
                                <option key={t.address} value={i} disabled={i === outputToken}>
                                    {t.symbol}
                                </option>
                            ))}
                        </select>
                        <input
                            type="number"
                            value={inputAmount}
                            onChange={(e) => setInputAmount(e.target.value)}
                            placeholder="0.0"
                            className="flex-[2] px-3 py-2 bg-background border border-border rounded-lg text-sm"
                        />
                    </div>
                </div>

                {/* Arrow */}
                <div className="flex justify-center">
                    <ArrowDown className="h-6 w-6 text-muted-foreground" />
                </div>

                {/* Output Token */}
                <div>
                    <label className="block text-sm font-medium mb-2">You Receive (Minimum)</label>
                    <div className="flex gap-2">
                        <select
                            value={outputToken}
                            onChange={(e) => setOutputToken(Number(e.target.value))}
                            className="flex-1 px-3 py-2 bg-secondary border border-border rounded-lg text-sm"
                        >
                            {tokens.map((t, i) => (
                                <option key={t.address} value={i} disabled={i === inputToken}>
                                    {t.symbol}
                                </option>
                            ))}
                        </select>
                        <input
                            type="number"
                            value={minOutput}
                            onChange={(e) => setMinOutput(e.target.value)}
                            placeholder="0.0"
                            className="flex-[2] px-3 py-2 bg-background border border-border rounded-lg text-sm"
                        />
                    </div>
                </div>

                {/* Deadline */}
                <div>
                    <label className="block text-sm font-medium mb-2">Deadline</label>
                    <select
                        value={deadline}
                        onChange={(e) => setDeadline(e.target.value)}
                        className="w-full px-3 py-2 bg-secondary border border-border rounded-lg text-sm"
                    >
                        <option value="1">1 hour</option>
                        <option value="6">6 hours</option>
                        <option value="24">24 hours</option>
                        <option value="168">7 days</option>
                    </select>
                </div>

                {/* Actions */}
                <div className="flex gap-2">
                    <button
                        onClick={handleApprove}
                        disabled={!inputAmount || isApproving}
                        className="flex-1 px-4 py-3 border border-border rounded-lg text-sm font-medium hover:bg-accent transition-colors disabled:opacity-50"
                    >
                        {isApproving ? (
                            <Loader2 className="h-4 w-4 animate-spin mx-auto" />
                        ) : (
                            '1. Approve'
                        )}
                    </button>
                    <button
                        onClick={handleCreate}
                        disabled={!inputAmount || !minOutput || isCreating}
                        className="flex-1 px-4 py-3 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors disabled:opacity-50"
                    >
                        {isCreating ? (
                            <Loader2 className="h-4 w-4 animate-spin mx-auto" />
                        ) : (
                            '2. Create Intent'
                        )}
                    </button>
                </div>
            </div>
        </div>
    )
}
