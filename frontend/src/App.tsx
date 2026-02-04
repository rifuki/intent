import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { WagmiProvider } from 'wagmi'
import { RouterProvider } from '@tanstack/react-router'

import { config } from '@/config/wagmi'
import { router } from '@/config/router'

const queryClient = new QueryClient()

export function App() {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>
    </WagmiProvider>
  )
}
