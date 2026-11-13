#!/usr/bin/env bash
# AetherWall launch script (development)
# Launch the Studio and daemon in development mode.
set -euo pipefail

cd "/c/Users/Rudy0senpai/T450/goto/git/AetherWall-Wallpaper engine"

echo "Launching AetherWall Studio (development mode)"

echo "Starting daemon..."
cargo run -p aetherwall-daemon -- --dev > /tmp/aetherwall-daemon.log 2>&1 &
DAEMON_PID=$!
echo "Daemon PID: $DAEMON_PID"
echo "Daemon will stay running even if the Studio closes."

# Wait for daemon to be ready
sleep 2

# Launch the Qt Studio
cd apps/aetherwall-studio
if command -v cmake &> /dev/null; then
    cmake --build . --config Debug 2>&1 | tail -5
    ./build/AetherWallStudio
else
    echo "WARNING: cmake not found. Build the Studio manually."
    echo "Run: scripts/build.sh"
fi

# Kill the daemon when the Studio exits
kill $DAEMON_PID 2>/dev/null || true
wait $DAEMON_PID 2>/dev/null || true
