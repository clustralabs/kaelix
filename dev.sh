#!/usr/bin/env bash
set -e

# Resolve repository root
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

# Ensure .env exists
if [ ! -f .env ]; then
  if [ -f .env.example ]; then
    echo "Creating .env from .env.example..."
    cp .env.example .env
  fi
fi

# Export environment variables from .env if present
if [ -f .env ]; then
  set -a
  # shellcheck disable=SC1091
  source .env
  set +a
fi

# Detect frontend runner (prefer bun, fallback to npm)
if command -v bun >/dev/null 2>&1; then
  JS_CMD="bun dev"
elif command -v npm >/dev/null 2>&1; then
  JS_CMD="npm run dev"
else
  echo "Error: neither bun nor npm found. Please install bun or Node.js." >&2
  exit 1
fi

# Ensure target ports are free
for PORT in "${BACKEND_PORT:-8000}" "${FRONTEND_PORT:-3000}"; do
  STALE_PIDS=$(lsof -ti :"$PORT" 2>/dev/null || true)
  if [ -n "$STALE_PIDS" ]; then
    echo "Freeing occupied port $PORT (PID: $STALE_PIDS)..."
    kill -9 $STALE_PIDS 2>/dev/null || true
  fi
done

# Clean shutdown handler for all child processes
BACKEND_PID=""
FRONTEND_PID=""

cleanup() {
  echo ""
  echo "Stopping all services..."
  trap - INT TERM EXIT
  if [ -n "$BACKEND_PID" ] && kill -0 "$BACKEND_PID" 2>/dev/null; then
    kill "$BACKEND_PID" 2>/dev/null || true
  fi
  if [ -n "$FRONTEND_PID" ] && kill -0 "$FRONTEND_PID" 2>/dev/null; then
    kill "$FRONTEND_PID" 2>/dev/null || true
  fi
  wait 2>/dev/null
  echo "All services stopped."
}

trap cleanup INT TERM EXIT

echo "========================================"
echo " Starting kaelix services"
echo " Backend:  http://localhost:${BACKEND_PORT:-8000}"
echo " Frontend: http://localhost:${FRONTEND_PORT:-3000}"
echo " Press Ctrl+C to stop all services"
echo "========================================"

# Start backend
(cd "$ROOT_DIR/Rust" && cargo run) &
BACKEND_PID=$!

# Start frontend
(cd "$ROOT_DIR/JS" && $JS_CMD) &
FRONTEND_PID=$!

# Wait for both processes
wait
