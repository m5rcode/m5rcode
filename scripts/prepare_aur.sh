#!/bin/bash
# Quick AUR setup script

set -e

echo "==> Preparing m5rcode for AUR submission"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Run this from the m5rcode root directory"
    exit 1
fi

VERSION="0.1.0"

# Create clean tarball
echo "==> Creating source tarball..."
cd ..
tar czf m5rcode-${VERSION}.tar.gz m5rcode/ \
    --exclude=m5rcode/target \
    --exclude=m5rcode/.git \
    --exclude=m5rcode/release \
    --exclude=m5rcode/Cargo.lock

echo "==> Tarball created: ../m5rcode-${VERSION}.tar.gz"
echo ""

# Generate checksum
echo "==> Generating SHA256 checksum..."
CHECKSUM=$(sha256sum m5rcode-${VERSION}.tar.gz | awk '{print $1}')
echo "SHA256: $CHECKSUM"
echo ""

# Update PKGBUILD
cd m5rcode/arch
echo "==> Updating PKGBUILD with checksum..."
sed -i "s/sha256sums=('SKIP')/sha256sums=('$CHECKSUM')/" PKGBUILD

# Generate .SRCINFO
echo "==> Generating .SRCINFO..."
makepkg --printsrcinfo > .SRCINFO

echo ""
echo "==> Setup complete!"
echo ""
echo "Next steps:"
echo "1. Upload tarball to GitHub releases or your server"
echo "2. Update PKGBUILD source URL if needed"
echo "3. Test build: cd arch && makepkg -si"
echo "4. Clone AUR repo: git clone ssh://aur@aur.archlinux.org/m5rcode.git"
echo "5. Copy PKGBUILD, .SRCINFO, m5rcode.install to AUR repo"
echo "6. Commit and push to AUR"
echo ""
echo "See arch/AUR_SUBMISSION.md for detailed instructions"
