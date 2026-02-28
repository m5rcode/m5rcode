#!/bin/bash
# Release script - creates release artifacts

set -e

VERSION="0.1.0"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Creating release for m5rcode v$VERSION"

cd "$PROJECT_ROOT"

# Clean previous builds
echo "==> Cleaning previous builds..."
cargo clean
rm -rf target/release

# Build all components
echo "==> Building release binaries..."
./scripts/build_arch.sh

# Run tests
echo "==> Running tests..."
cd compiler
cargo test --release
cd ..

# Create release directory
RELEASE_DIR="$PROJECT_ROOT/release/m5rcode-$VERSION"
mkdir -p "$RELEASE_DIR"

# Copy binaries
echo "==> Copying binaries..."
mkdir -p "$RELEASE_DIR/bin"
cp compiler/target/release/m5r "$RELEASE_DIR/bin/"
cp tools/m5repl/target/release/m5repl "$RELEASE_DIR/bin/"
cp tools/m5idle/target/release/m5idle "$RELEASE_DIR/bin/"
cp target/release/m5fmt "$RELEASE_DIR/bin/"
cp target/release/m5lint "$RELEASE_DIR/bin/"
cp tools/lsp/target/release/m5lsp "$RELEASE_DIR/bin/"

# Copy documentation
echo "==> Copying documentation..."
cp README.md "$RELEASE_DIR/"
cp LICENSE "$RELEASE_DIR/"
cp -r SPEC "$RELEASE_DIR/"
cp -r docs "$RELEASE_DIR/"
cp -r packages "$RELEASE_DIR/examples"

# Create tarball
echo "==> Creating tarball..."
cd "$PROJECT_ROOT/release"
tar czf "m5rcode-$VERSION-x86_64.tar.gz" "m5rcode-$VERSION"

echo "==> Release created: release/m5rcode-$VERSION-x86_64.tar.gz"
echo ""
echo "Contents:"
ls -lh "m5rcode-$VERSION-x86_64.tar.gz"
