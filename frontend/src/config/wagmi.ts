import { http, createConfig } from 'wagmi'
import { base, baseSepolia } from 'wagmi/chains'
import { injected, walletConnect } from 'wagmi/connectors'

// WalletConnect project ID - get from https://cloud.walletconnect.com
const projectId = import.meta.env.VITE_WALLETCONNECT_PROJECT_ID || ''

export const config = createConfig({
    chains: [base, baseSepolia],
    connectors: [
        injected(),
        ...(projectId ? [walletConnect({ projectId })] : []),
    ],
    transports: {
        [base.id]: http(),
        [baseSepolia.id]: http('https://sepolia.base.org'),
    },
})

declare module 'wagmi' {
    interface Register {
        config: typeof config
    }
}
