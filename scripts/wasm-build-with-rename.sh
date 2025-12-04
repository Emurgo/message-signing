#!/bin/bash

# Script to build WASM with temporarily renamed package
# Usage: ./wasm-build-with-rename.sh <target> [--gc]
# Example: ./wasm-build-with-rename.sh nodejs
# Example: ./wasm-build-with-rename.sh browser --gc

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CARGO_TOML="$PROJECT_ROOT/rust/Cargo.toml"
RUST_DIR="$PROJECT_ROOT/rust"

OLD_NAME="emurgo-cardano-message-signing"
NEW_NAME="cardano-message-signing"

# Parse arguments
TARGET=""
GC_FLAG=""

for arg in "$@"; do
    case $arg in
        --gc)
            GC_FLAG="1"
            ;;
        *)
            TARGET="$arg"
            ;;
    esac
done

if [ -z "$TARGET" ]; then
    echo "Usage: $0 <target> [--gc]"
    echo "  target: nodejs, browser, web"
    echo "  --gc: enable weak references (WASM_BINDGEN_WEAKREF=1)"
    exit 1
fi

# Function to restore original Cargo.toml on exit (success or failure)
cleanup() {
    if [ -f "$CARGO_TOML.bak" ]; then
        echo "Restoring original Cargo.toml..."
        mv "$CARGO_TOML.bak" "$CARGO_TOML"
    fi
}

# Set up trap to restore on exit, interrupt, or error
trap cleanup EXIT INT TERM

# Check if Cargo.toml exists
if [ ! -f "$CARGO_TOML" ]; then
    echo "Error: $CARGO_TOML not found"
    exit 1
fi

# Backup original Cargo.toml
echo "Backing up Cargo.toml..."
cp "$CARGO_TOML" "$CARGO_TOML.bak"

# Replace package name
echo "Renaming package from '$OLD_NAME' to '$NEW_NAME'..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/name = \"$OLD_NAME\"/name = \"$NEW_NAME\"/" "$CARGO_TOML"
else
    # Linux
    sed -i "s/name = \"$OLD_NAME\"/name = \"$NEW_NAME\"/" "$CARGO_TOML"
fi

# Verify the change was made
if grep -q "name = \"$NEW_NAME\"" "$CARGO_TOML"; then
    echo "Package name successfully changed."
else
    echo "Error: Failed to change package name"
    exit 1
fi

# Build
cd "$RUST_DIR"

if [ -n "$GC_FLAG" ]; then
    echo "Building with WASM_BINDGEN_WEAKREF=1 --target=$TARGET..."
    WASM_BINDGEN_WEAKREF=1 wasm-pack build --target="$TARGET"
else
    echo "Building with --target=$TARGET..."
    wasm-pack build --target="$TARGET"
fi

echo "Packing..."
wasm-pack pack

echo "Build completed successfully!"
# cleanup will be called automatically due to trap
