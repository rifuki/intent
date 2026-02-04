import { Link } from '@tanstack/react-router'
import { ArrowRight, Zap, Shield, Clock } from 'lucide-react'
import { useAccount } from 'wagmi'

export function HomePage() {
    const { isConnected } = useAccount()

    return (
        <div className="max-w-4xl mx-auto">
            {/* Hero */}
            <section className="text-center py-16">
                <h1 className="text-4xl font-bold tracking-tight mb-4">
                    Swap with Intent
                </h1>
                <p className="text-lg text-muted-foreground mb-8 max-w-2xl mx-auto">
                    Declare what you want, not how. Create intents and let solvers compete
                    to give you the best execution.
                </p>
                <div className="flex items-center justify-center gap-4">
                    {isConnected ? (
                        <Link
                            to="/create"
                            className="inline-flex items-center gap-2 px-6 py-3 bg-primary text-primary-foreground rounded-lg font-medium hover:bg-primary/90 transition-colors"
                        >
                            Create Intent
                            <ArrowRight className="h-4 w-4" />
                        </Link>
                    ) : (
                        <p className="text-sm text-muted-foreground">
                            Connect your wallet to get started
                        </p>
                    )}
                </div>
            </section>

            {/* Features */}
            <section className="grid md:grid-cols-3 gap-6 py-12">
                <div className="p-6 border border-border rounded-lg">
                    <Zap className="h-8 w-8 mb-4 text-primary" />
                    <h3 className="font-semibold mb-2">Competitive Execution</h3>
                    <p className="text-sm text-muted-foreground">
                        Solvers compete to fill your intent, ensuring you get the best rate.
                    </p>
                </div>
                <div className="p-6 border border-border rounded-lg">
                    <Shield className="h-8 w-8 mb-4 text-primary" />
                    <h3 className="font-semibold mb-2">Protected Swaps</h3>
                    <p className="text-sm text-muted-foreground">
                        Set minimum output amounts. Your intent won't execute below your limit.
                    </p>
                </div>
                <div className="p-6 border border-border rounded-lg">
                    <Clock className="h-8 w-8 mb-4 text-primary" />
                    <h3 className="font-semibold mb-2">Flexible Deadlines</h3>
                    <p className="text-sm text-muted-foreground">
                        Set your deadline. Cancel anytime after expiry if not filled.
                    </p>
                </div>
            </section>

            {/* How it works */}
            <section className="py-12 border-t border-border">
                <h2 className="text-2xl font-bold text-center mb-8">How It Works</h2>
                <div className="grid md:grid-cols-4 gap-4">
                    <div className="text-center">
                        <div className="w-10 h-10 rounded-full bg-primary text-primary-foreground flex items-center justify-center mx-auto mb-3 font-semibold">
                            1
                        </div>
                        <h4 className="font-medium mb-1">Create Intent</h4>
                        <p className="text-xs text-muted-foreground">
                            Specify tokens and minimum output
                        </p>
                    </div>
                    <div className="text-center">
                        <div className="w-10 h-10 rounded-full bg-primary text-primary-foreground flex items-center justify-center mx-auto mb-3 font-semibold">
                            2
                        </div>
                        <h4 className="font-medium mb-1">Deposit Tokens</h4>
                        <p className="text-xs text-muted-foreground">
                            Your input tokens go to the contract
                        </p>
                    </div>
                    <div className="text-center">
                        <div className="w-10 h-10 rounded-full bg-primary text-primary-foreground flex items-center justify-center mx-auto mb-3 font-semibold">
                            3
                        </div>
                        <h4 className="font-medium mb-1">Solver Fills</h4>
                        <p className="text-xs text-muted-foreground">
                            Solver executes your swap atomically
                        </p>
                    </div>
                    <div className="text-center">
                        <div className="w-10 h-10 rounded-full bg-primary text-primary-foreground flex items-center justify-center mx-auto mb-3 font-semibold">
                            4
                        </div>
                        <h4 className="font-medium mb-1">Receive Tokens</h4>
                        <p className="text-xs text-muted-foreground">
                            Output tokens sent directly to you
                        </p>
                    </div>
                </div>
            </section>
        </div>
    )
}
