import { useAccount, useConnect, useDisconnect, useChainId, useSwitchChain } from 'wagmi'
import { ChevronDown, LogOut, Copy, Check, Wallet } from 'lucide-react'
import { useState } from 'react'
import { baseSepolia, base } from 'wagmi/chains'

export function ConnectButton() {
    const { address, isConnected } = useAccount()
    const { connect, connectors } = useConnect()
    const { disconnect } = useDisconnect()
    const chainId = useChainId()
    const { switchChain } = useSwitchChain()

    const [copied, setCopied] = useState(false)
    const [open, setOpen] = useState(false)

    const copyAddress = () => {
        if (address) {
            navigator.clipboard.writeText(address)
            setCopied(true)
            setTimeout(() => setCopied(false), 2000)
        }
    }

    const shortenAddress = (addr: string) => {
        return `${addr.slice(0, 6)}...${addr.slice(-4)}`
    }

    if (!isConnected) {
        return (
            <div className="relative">
                <button
                    onClick={() => setOpen(!open)}
                    className="inline-flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors"
                >
                    <Wallet className="h-4 w-4" />
                    Connect Wallet
                    <ChevronDown className="h-4 w-4" />
                </button>

                {open && (
                    <div className="absolute right-0 mt-2 w-56 bg-card border border-border rounded-lg shadow-lg p-2 z-50">
                        {connectors.map((connector) => (
                            <button
                                key={connector.uid}
                                onClick={() => {
                                    connect({ connector })
                                    setOpen(false)
                                }}
                                className="w-full text-left px-3 py-2 text-sm rounded-md hover:bg-accent transition-colors"
                            >
                                {connector.name}
                            </button>
                        ))}
                    </div>
                )}
            </div>
        )
    }

    return (
        <div className="relative">
            <button
                onClick={() => setOpen(!open)}
                className="inline-flex items-center gap-2 px-4 py-2 bg-secondary text-secondary-foreground rounded-lg text-sm font-medium hover:bg-secondary/80 transition-colors"
            >
                <span className="w-2 h-2 bg-green-500 rounded-full" />
                {shortenAddress(address!)}
                <ChevronDown className="h-4 w-4" />
            </button>

            {open && (
                <div className="absolute right-0 mt-2 w-56 bg-card border border-border rounded-lg shadow-lg p-2 z-50">
                    {/* Network switcher */}
                    <div className="px-3 py-2 text-xs text-muted-foreground border-b border-border mb-2">
                        Network
                    </div>
                    <button
                        onClick={() => switchChain?.({ chainId: baseSepolia.id })}
                        className={`w-full text-left px-3 py-2 text-sm rounded-md transition-colors ${chainId === baseSepolia.id ? 'bg-accent' : 'hover:bg-accent'
                            }`}
                    >
                        Base Sepolia {chainId === baseSepolia.id && '✓'}
                    </button>
                    <button
                        onClick={() => switchChain?.({ chainId: base.id })}
                        className={`w-full text-left px-3 py-2 text-sm rounded-md transition-colors ${chainId === base.id ? 'bg-accent' : 'hover:bg-accent'
                            }`}
                    >
                        Base Mainnet {chainId === base.id && '✓'}
                    </button>

                    <div className="border-t border-border my-2" />

                    {/* Copy address */}
                    <button
                        onClick={copyAddress}
                        className="w-full flex items-center gap-2 px-3 py-2 text-sm rounded-md hover:bg-accent transition-colors"
                    >
                        {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
                        {copied ? 'Copied!' : 'Copy Address'}
                    </button>

                    {/* Disconnect */}
                    <button
                        onClick={() => {
                            disconnect()
                            setOpen(false)
                        }}
                        className="w-full flex items-center gap-2 px-3 py-2 text-sm rounded-md hover:bg-destructive/10 text-destructive transition-colors"
                    >
                        <LogOut className="h-4 w-4" />
                        Disconnect
                    </button>
                </div>
            )}
        </div>
    )
}
