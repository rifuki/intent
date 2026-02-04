import { createRouter, createRootRoute, createRoute } from '@tanstack/react-router'

import { RootLayout } from '@/components/layout/RootLayout'
import { HomePage } from '@/routes/HomePage'
import { CreateIntentPage } from '@/routes/CreateIntentPage'
import { MyIntentsPage } from '@/routes/MyIntentsPage'

// Root route with layout
const rootRoute = createRootRoute({
    component: RootLayout,
})

// Home route
const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/',
    component: HomePage,
})

// Create intent route
const createRoute_ = createRoute({
    getParentRoute: () => rootRoute,
    path: '/create',
    component: CreateIntentPage,
})

// My intents route  
const myIntentsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: '/my-intents',
    component: MyIntentsPage,
})

// Route tree
const routeTree = rootRoute.addChildren([
    indexRoute,
    createRoute_,
    myIntentsRoute,
])

// Router instance
export const router = createRouter({ routeTree })

// Type declaration for router
declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router
    }
}
