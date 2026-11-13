#!/usr/bin/env bash
# AetherWall build script (development)
# Requires: Rust toolchain, Qt 6, CMake
set -euo pipefail

echo "AetherWall build script"
echo "======================="
echo ""

cd "/c/Users/Rudy0senpai/T450/goto/git/AetherWall-Wallpaper engine"

echo "1/4 Building Rust workspace..."
cargo build --workspace --all-targets 2>&1 | tail -20
echo "Rust build: done"

echo "2/4 Building runtimes..."
cargo build -p aetherwall-runtime-image 2>&1 | tail -10
cargo build -p aetherwall-runtime-slideshow 2>&1 | tail -10
echo "Runtimes build: done"

echo "3/4 Configuring Qt Studio build..."
cd apps/aetherwall-studio
if ! command -v cmake &> /dev/null; then
    echo "ERROR: cmake not found. Install CMake: see docs/phase-1/environment-audit.md"
    exit 1
fi
cmake . 2>&1 | tail -10
echo "Qt studio configure: done"

echo "4/4 Building Studio..."
cmake --build . 2>&1 | tail -20
echo "Studio build: done"

echo ""
echo "Build complete:"
echo "  Binary: apps/aetherwall-studio/build/AetherWallStudio"
echo "  Daemon: target/release/aetherwall-daemon"
echo ""
echo "Launch with: scripts/launch.sh"
