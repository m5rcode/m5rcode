# m5rcode AUR Package

This directory contains files for publishing m5rcode to the Arch User Repository (AUR).

## Prerequisites

1. AUR account: https://aur.archlinux.org/register
2. SSH key added to AUR account
3. Git configured with your name and email

## Initial AUR Submission

### 1. Create source tarball

```bash
cd /home/m5rcel
tar czf m5rcode-0.1.0.tar.gz m5rcode/ \
    --exclude=m5rcode/target \
    --exclude=m5rcode/.git \
    --exclude=m5rcode/release
```

### 2. Generate checksums

```bash
cd /home/m5rcel/m5rcode/arch
sha256sum ../../m5rcode-0.1.0.tar.gz
```

Update the `sha256sums` line in PKGBUILD with the output.

### 3. Test the package locally

```bash
cd /home/m5rcel/m5rcode/arch
makepkg -si
```

### 4. Generate .SRCINFO

```bash
makepkg --printsrcinfo > .SRCINFO
```

### 5. Clone AUR repository

```bash
git clone ssh://aur@aur.archlinux.org/m5rcode.git m5rcode-aur
cd m5rcode-aur
```

### 6. Copy files

```bash
cp ../m5rcode/arch/PKGBUILD .
cp ../m5rcode/arch/m5rcode.install .
makepkg --printsrcinfo > .SRCINFO
```

### 7. Commit and push

```bash
git add PKGBUILD m5rcode.install .SRCINFO
git commit -m "Initial commit: m5rcode 0.1.0"
git push
```

## Updating the Package

When releasing a new version:

1. Update version in PKGBUILD
2. Update pkgrel to 1
3. Create new tarball
4. Update sha256sums
5. Test build
6. Regenerate .SRCINFO
7. Commit and push

```bash
# Example update
cd m5rcode-aur
# Edit PKGBUILD (update pkgver)
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.2.0"
git push
```

## Hosting the Source Tarball

You need to host the source tarball somewhere accessible. Options:

### Option 1: GitHub Releases (Recommended)

1. Create GitHub repository: https://github.com/new
2. Push code: `git push origin main`
3. Create release: https://github.com/YOUR_USERNAME/m5rcode/releases/new
4. Upload tarball as release asset
5. Update PKGBUILD source URL:

```bash
source=("https://github.com/YOUR_USERNAME/m5rcode/archive/v${pkgver}.tar.gz")
```

### Option 2: Your Own Server

Upload tarball to your server and update PKGBUILD:

```bash
source=("https://your-domain.com/releases/m5rcode-${pkgver}.tar.gz")
```

## PKGBUILD Template for GitHub

```bash
# Maintainer: Your Name <your.email@example.com>
pkgname=m5rcode
pkgver=0.1.0
pkgrel=1
pkgdesc="Modern programming language combining Python, C, Java, HolyC, Rust, and Ruby"
arch=('x86_64')
url="https://github.com/YOUR_USERNAME/m5rcode"
license=('MIT' 'Apache')
depends=('gcc-libs' 'glibc')
makedepends=('rust' 'cargo' 'pkg-config')
optdepends=(
    'gtk3: for m5idle GUI'
    'llvm: for LLVM backend'
)
install=m5rcode.install
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/YOUR_USERNAME/m5rcode/archive/v${pkgver}.tar.gz")
sha256sums=('SKIP')  # Update after first build

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
    rustc --edition 2021 -O tools/m5fmt.rs -o target/release/m5fmt
    rustc --edition 2021 -O tools/m5lint.rs -o target/release/m5lint
}

check() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo test --release
    ./target/release/m5repl packages/hello_world.m5 || exit 1
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    
    install -Dm755 m5rcode "$pkgdir/usr/bin/m5rcode"
    install -Dm755 compiler/target/release/m5r "$pkgdir/usr/bin/m5r"
    install -Dm755 tools/m5repl/target/release/m5repl "$pkgdir/usr/bin/m5repl"
    install -Dm755 tools/m5idle/target/release/m5idle "$pkgdir/usr/bin/m5idle"
    install -Dm755 target/release/m5fmt "$pkgdir/usr/bin/m5fmt"
    install -Dm755 target/release/m5lint "$pkgdir/usr/bin/m5lint"
    install -Dm755 tools/lsp/target/release/m5lsp "$pkgdir/usr/bin/m5lsp"
    
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

## Testing Before Submission

```bash
# Clean build test
cd /home/m5rcel/m5rcode/arch
rm -rf src pkg *.tar.gz
makepkg -si

# Verify installation
m5rcode version
m5rcode run /usr/share/doc/m5rcode/examples/hello_world.m5
```

## AUR Package Guidelines

- Follow Arch packaging standards: https://wiki.archlinux.org/title/Arch_package_guidelines
- Use proper dependencies
- Include .SRCINFO
- Test on clean Arch system
- Respond to comments promptly
- Keep package updated

## Useful Commands

```bash
# Check PKGBUILD syntax
namcap PKGBUILD

# Check built package
namcap m5rcode-*.pkg.tar.zst

# Clean build directory
makepkg -c

# Update checksums automatically
updpkgsums
```

## Support

After publishing to AUR, users can install with:

```bash
# Using yay
yay -S m5rcode

# Using paru
paru -S m5rcode

# Manual
git clone https://aur.archlinux.org/m5rcode.git
cd m5rcode
makepkg -si
```

## Maintenance

- Monitor AUR comments
- Update for new releases
- Fix reported issues
- Keep dependencies current
- Test on latest Arch

## Links

- AUR: https://aur.archlinux.org/
- AUR Guidelines: https://wiki.archlinux.org/title/AUR_submission_guidelines
- Packaging Standards: https://wiki.archlinux.org/title/Arch_package_guidelines
