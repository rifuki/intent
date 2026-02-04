import { Outlet, Link } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import { ArrowLeftRight, Home, List } from 'lucide-react'

import { ConnectButton } from '@/components/layout/ConnectButton'

export function RootLayout() {
    return (
        <div className="min-h-screen bg-background">
            {/* Header */}
            <header className="border-b border-border">
                <div className="container mx-auto px-4 h-16 flex items-center justify-between">
                    {/* Logo */}
                    <Link to="/" className="flex items-center gap-2 font-semibold text-lg">
                        <ArrowLeftRight className="h-5 w-5" />
                        <span>Intent Swap</span>
                    </Link>

                    {/* Navigation */}
                    <nav className="hidden md:flex items-center gap-6">
                        <Link
                            to="/"
                            className="text-sm text-muted-foreground hover:text-foreground transition-colors [&.active]:text-foreground"
                        >
                            <Home className="inline-block h-4 w-4 mr-1" />
                            Home
                        </Link>
                        <Link
                            to="/create"
                            className="text-sm text-muted-foreground hover:text-foreground transition-colors [&.active]:text-foreground"
                        >
                            Create Intent
                        </Link>
                        <Link
                            to="/my-intents"
                            className="text-sm text-muted-foreground hover:text-foreground transition-colors [&.active]:text-foreground"
                        >
                            <List className="inline-block h-4 w-4 mr-1" />
                            My Intents
                        </Link>
                    </nav>

                    {/* Connect Button */}
                    <ConnectButton />
                </div>
            </header>

            {/* Main content */}
            <main className="container mx-auto px-4 py-8">
                <Outlet />
            </main>

            {/* Footer */}
            <footer className="border-t border-border mt-auto">
                <div className="container mx-auto px-4 py-6 text-center text-sm text-muted-foreground">
                    Intent Swap &middot; Base Sepolia Testnet
                </div>
            </footer>

            {/* Dev tools */}
            <TanStackRouterDevtools />
        </div>
    )
}
