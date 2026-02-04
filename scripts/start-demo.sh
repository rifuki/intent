#!/bin/bash

# Trap Ctrl+C to kill all child processes
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

echo "🚀 Starting Intent System Demo..."

# 1. Start API Server
echo "🌐 Starting API Server..."
cargo run -p intent-api &
API_PID=$!

# 2. Start Solver
echo "🤖 Starting Solver..."
cargo run -p intent-solver &
SOLVER_PID=$!

# 3. Start Frontend
echo "🎨 Starting Frontend..."
cd frontend
npm run dev &
FRONTEND_PID=$!

# Wait for all processes
wait $API_PID $SOLVER_PID $FRONTEND_PID
