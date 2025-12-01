#!/bin/bash
# Build script for Windows deployment from Mac

set -e

echo "OpenGolfCoach Windows Demo - Build Script"
echo "=========================================="
echo ""

# Check if we're on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS - checking for MinGW toolchain..."

    # Check if mingw-w64 is installed
    if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        echo ""
        echo "MinGW toolchain not found. Installing via Homebrew..."
        echo ""

        if ! command -v brew &> /dev/null; then
            echo "Error: Homebrew is required but not installed."
            echo "Install from: https://brew.sh"
            exit 1
        fi

        echo "Running: brew install mingw-w64"
        brew install mingw-w64
    fi

    TARGET="x86_64-pc-windows-gnu"
else
    TARGET="x86_64-pc-windows-gnu"
fi

# Check if Windows target is installed
if ! rustup target list | grep -q "$TARGET (installed)"; then
    echo "Installing Rust Windows target: $TARGET..."
    rustup target add $TARGET
fi

echo ""
echo "Building for Windows (release mode)..."
echo "Target: $TARGET"
echo ""

# Get the directory where this script lives
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$SCRIPT_DIR"
cargo build --release --target $TARGET --target-dir target

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ Build complete!"
    echo ""

    # Extract version from workspace Cargo.toml
    VERSION=$(grep '^version = ' ../Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

    # Create versioned filename
    VERSIONED_NAME="opengolfcoach-windows-demo-v${VERSION}.exe"

    # Copy to versioned filename
    cp "target/$TARGET/release/opengolfcoach-windows-demo.exe" "target/$TARGET/release/$VERSIONED_NAME"

    echo "Executable location:"
    echo "  target/$TARGET/release/opengolfcoach-windows-demo.exe"
    echo "  target/$TARGET/release/$VERSIONED_NAME"
    echo ""
    echo "File size:"
    ls -lh "target/$TARGET/release/$VERSIONED_NAME" | awk '{print "  "$5}'
    echo ""
    echo "To create a portable distribution:"
    echo "  1. Copy the versioned .exe file ($VERSIONED_NAME) to your distribution folder"
    echo "  2. Transfer to Windows machine"
    echo "  3. Run without installation!"
    echo ""
else
    echo ""
    echo "✗ Build failed!"
    echo ""
    echo "If you see MinGW errors on macOS, try:"
    echo "  brew install mingw-w64"
    echo ""
    echo "Alternative: Build on a Windows machine using build-windows.bat"
    exit 1
fi
