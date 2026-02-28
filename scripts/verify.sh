#!/bin/bash
# Verification script for m5rcode repository

set -e

echo "==> m5rcode Repository Verification"
echo ""

# Check directory structure
echo "✓ Checking directory structure..."
for dir in SPEC compiler runtime tools packages arch scripts docs ci tests; do
    if [ -d "$dir" ]; then
        echo "  ✓ $dir/"
    else
        echo "  ✗ $dir/ MISSING"
        exit 1
    fi
done

# Check key files
echo ""
echo "✓ Checking key files..."
for file in README.md LICENSE Cargo.toml CONTRIBUTING.md SECURITY.md; do
    if [ -f "$file" ]; then
        echo "  ✓ $file"
    else
        echo "  ✗ $file MISSING"
        exit 1
    fi
done

# Check SPEC files
echo ""
echo "✓ Checking SPEC files..."
for file in language_overview.md grammar.ebnf typing_spec.md semantics.md stdlib_spec.md; do
    if [ -f "SPEC/$file" ]; then
        echo "  ✓ SPEC/$file"
    else
        echo "  ✗ SPEC/$file MISSING"
        exit 1
    fi
done

# Build check
echo ""
echo "✓ Building project..."
cargo build --release > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "  ✓ Cargo build successful"
else
    echo "  ✗ Cargo build failed"
    exit 1
fi

# Check binaries
echo ""
echo "✓ Checking binaries..."
for bin in m5r m5repl m5idle m5lsp; do
    if [ -f "target/release/$bin" ]; then
        echo "  ✓ target/release/$bin"
    else
        echo "  ✗ target/release/$bin MISSING"
        exit 1
    fi
done

# Build standalone tools
echo ""
echo "✓ Building standalone tools..."
rustc --edition 2021 -O tools/m5fmt.rs -o target/release/m5fmt 2>/dev/null
rustc --edition 2021 -O tools/m5lint.rs -o target/release/m5lint 2>/dev/null
if [ -f "target/release/m5fmt" ] && [ -f "target/release/m5lint" ]; then
    echo "  ✓ m5fmt and m5lint built"
else
    echo "  ✗ Failed to build standalone tools"
    exit 1
fi

# Run tests
echo ""
echo "✓ Running tests..."
cargo test --release > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "  ✓ All tests passed"
else
    echo "  ✗ Tests failed"
    exit 1
fi

# Test hello world
echo ""
echo "✓ Testing hello world..."
output=$(./target/release/m5repl packages/hello_world.m5 2>&1)
if [ "$output" = "Hello, m5rcode!" ]; then
    echo "  ✓ Hello world works"
else
    echo "  ✗ Hello world failed"
    echo "  Expected: Hello, m5rcode!"
    echo "  Got: $output"
    exit 1
fi

# Check documentation
echo ""
echo "✓ Checking documentation..."
for file in tutorial.md api_reference.md examples.md; do
    if [ -f "docs/$file" ]; then
        echo "  ✓ docs/$file"
    else
        echo "  ✗ docs/$file MISSING"
        exit 1
    fi
done

# Check examples
echo ""
echo "✓ Checking examples..."
if [ -f "packages/hello_world.m5" ]; then
    echo "  ✓ packages/hello_world.m5"
else
    echo "  ✗ packages/hello_world.m5 MISSING"
    exit 1
fi

# Check Arch packaging
echo ""
echo "✓ Checking Arch packaging..."
if [ -f "arch/PKGBUILD" ]; then
    echo "  ✓ arch/PKGBUILD"
else
    echo "  ✗ arch/PKGBUILD MISSING"
    exit 1
fi

# Check scripts
echo ""
echo "✓ Checking scripts..."
for script in bootstrap.sh build_arch.sh make_release.sh; do
    if [ -x "scripts/$script" ]; then
        echo "  ✓ scripts/$script (executable)"
    else
        echo "  ✗ scripts/$script (not executable or missing)"
        exit 1
    fi
done

# Summary
echo ""
echo "========================================="
echo "✓ All verification checks passed!"
echo "========================================="
echo ""
echo "Repository is complete and buildable."
echo ""
echo "Quick start:"
echo "  ./target/release/m5repl                    # Start REPL"
echo "  ./target/release/m5repl hello.m5           # Run script"
echo "  ./target/release/m5r hello.m5              # Compile script"
echo "  ./scripts/build_arch.sh --install          # Install on Arch"
echo ""
