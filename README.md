# Intent System

> Cross-chain intent-based swap system

## What is this?

A system where users declare **what they want** (e.g., "swap 100 USDC for at least 0.05 ETH") without specifying how. Solvers compete to fulfill these intents.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌──────────────────┐
│    User     │────▶│  API Server │────▶│  IntentGateway   │
│  (Frontend) │     │ (intent-api)│     │   (Contract)     │
└─────────────┘     └─────────────┘     └──────────────────┘
                                               ▲
                                               │
                    ┌─────────────┐            │
                    │   Solver    │────────────┘
                    │(intent-solver)│
                    └─────────────┘
```

## Project Structure

```
intent/
├── Cargo.toml              # Workspace config
├── intent-core/            # Core types & traits
├── intent-evm/             # EVM adapter (alloy-rs)
├── intent-api/             # HTTP API server (Axum)
├── intent-solver/          # Automated intent filler
├── contracts/evm/          # Solidity contracts (Foundry)
└── scripts/                # E2E test scripts
```

## Deployed Contracts

| Network | Contract | Address |
|---------|----------|---------|
| Base Sepolia | IntentGateway | `0x4D7Ec71a5bD4Fcf7D56A3679518FDC55a8311683` |

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run API server
cd intent-api && cargo run

# Run solver (separate terminal)
cd intent-solver && cargo run
```

## Configuration

### API Server (`intent-api/.env`)
```env
PORT=8000
CORS_ALLOWED_ORIGINS=http://localhost:5173
LOG_LEVEL=debug

# Base Sepolia
BASE_SEPOLIA_RPC=https://sepolia.base.org
BASE_SEPOLIA_GATEWAY=0x4D7Ec71a5bD4Fcf7D56A3679518FDC55a8311683
BASE_SEPOLIA_PRIVATE_KEY=0x...  # Optional, for signing
```

### Solver (`intent-solver/.env`)
```env
RPC_URL=https://sepolia.base.org
GATEWAY_ADDRESS=0x4D7Ec71a5bD4Fcf7D56A3679518FDC55a8311683
SOLVER_PRIVATE_KEY=0x...
MIN_PROFIT_BPS=50
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/health` | Health check |
| GET | `/api/v1/chains` | List supported chains |
| POST | `/api/v1/intents` | Create intent |
| GET | `/api/v1/intents/:id` | Get intent details |
| POST | `/api/v1/intents/:id/cancel` | Cancel intent |

## E2E Testing

```bash
cd scripts
cp .env.example .env
# Edit with your PRIVATE_KEY

# Check balances
./e2e-test.sh

# Create intent
./create-intent.sh

# Fill intent (as solver)
./fill-intent.sh 0
```

**Test Tokens (Base Sepolia):**
- USDC: `0x036CbD53842c5426634e7929541eC2318f3dCF7e`
- WETH: `0x4200000000000000000000000000000000000006`

**Faucets:**
- ETH: https://www.alchemy.com/faucets/base-sepolia
- USDC: https://faucet.circle.com/

## Tech Stack

- **Rust** — Core logic, API, solver
- **Solidity** — EVM contracts (Foundry)
- **Axum** — HTTP API framework
- **alloy-rs** — EVM interaction
- **tokio** — Async runtime

## Project Status

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1 | ✅ Done | Smart Contract (IntentGateway) |
| Phase 2 | ✅ Done | EVM Adapter (alloy-rs) |
| Phase 3 | ✅ Done | API Server |
| Phase 4 | ✅ Done | Solver |
| Phase 5 | ✅ Done | E2E Testing |

## Documentation

- **[SPEC.md](./SPEC.md)** — Full technical specification

## License

MIT
