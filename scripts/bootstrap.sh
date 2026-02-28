#!/bin/bash
# Bootstrap script - sets up development environment

set -e

echo "==> m5rcode Bootstrap Script"
echo ""

# Detect OS
if [[ -f /etc/arch-release ]]; then
    OS="arch"
elif [[ -f /etc/debian_version ]]; then
    OS="debian"
else
    OS="unknown"
fi

echo "Detected OS: $OS"
echo ""

# Install dependencies based on OS
case $OS in
    arch)
        echo "==> Installing Arch Linux dependencies..."
        sudo pacman -S --needed --noconfirm \
            rust \
            cargo \
            pkg-config \
            gtk3 \
            glib2 \
            llvm \
            clang \
            python \
            base-devel \
            git
        ;;
    debian)
        echo "==> Installing Debian/Ubuntu dependencies..."
        sudo apt-get update
        sudo apt-get install -y \
            curl \
            build-essential \
            pkg-config \
            libgtk-3-dev \
            libglib2.0-dev \
            llvm \
            clang \
            python3 \
            git
        
        # Install Rust if not present
        if ! command -v cargo &> /dev/null; then
            echo "==> Installing Rust..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "$HOME/.cargo/env"
        fi
        ;;
    *)
        echo "Unsupported OS. Please install dependencies manually:"
        echo "  - Rust and Cargo"
        echo "  - pkg-config"
        echo "  - GTK3 development files"
        echo "  - LLVM and Clang"
        echo "  - Python 3"
        exit 1
        ;;
esac

echo ""
echo "==> Verifying installation..."
cargo --version
rustc --version

echo ""
echo "==> Bootstrap complete!"
echo ""
echo "Next steps:"
echo "  1. Run './scripts/build_arch.sh' to build the project"
echo "  2. Run './scripts/build_arch.sh --install' to build and install"
echo "  3. Or use 'cargo build --release' in individual directories"
