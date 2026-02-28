#!/bin/bash
# Build script for Arch Linux

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "==> Installing build dependencies..."
sudo pacman -S --needed --noconfirm \
    rust \
    cargo \
    pkg-config \
    gtk3 \
    glib2 \
    llvm \
    clang \
    python \
    base-devel

echo "==> Building m5rcode..."
cd "$PROJECT_ROOT"

# Build compiler
echo "==> Building compiler..."
cd compiler
cargo build --release
cd ..

# Build REPL
echo "==> Building REPL..."
cd tools/m5repl
cargo build --release
cd ../..

# Build IDE
echo "==> Building IDE..."
cd tools/m5idle
cargo build --release
cd ../..

# Build formatter
echo "==> Building formatter..."
rustc --edition 2021 -O tools/m5fmt.rs -o target/release/m5fmt

# Build linter
echo "==> Building linter..."
rustc --edition 2021 -O tools/m5lint.rs -o target/release/m5lint

# Build LSP
echo "==> Building LSP..."
cd tools/lsp
cargo build --release
cd ../..

echo "==> Build complete!"
echo ""
echo "Binaries are in:"
echo "  compiler/target/release/m5r"
echo "  tools/m5repl/target/release/m5repl"
echo "  tools/m5idle/target/release/m5idle"
echo "  target/release/m5fmt"
echo "  target/release/m5lint"
echo "  tools/lsp/target/release/m5lsp"
echo ""

# Check if --install flag is provided
if [[ "$1" == "--install" ]]; then
    echo "==> Installing via makepkg..."
    cd "$PROJECT_ROOT"
    
    # Create tarball for PKGBUILD
    cd ..
    tar czf m5rcode-0.1.0.tar.gz m5rcode/
    mv m5rcode-0.1.0.tar.gz m5rcode/
    cd m5rcode
    
    # Build package
    cd arch
    makepkg -si --noconfirm
    
    echo "==> Installation complete!"
    echo "Run 'm5repl' to start the REPL"
fi
