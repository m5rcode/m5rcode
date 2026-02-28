# m5rcode

**m5rcode** (pronounced "em-five-er-code") is a modern programming language combining the best features of Python, C, Java, HolyC, Rust, and Ruby. It features gradual typing, hybrid memory management (GC + optional ownership), and a focus on developer productivity.

## ✨ What's New in v0.3.0

- **Comprehensive Standard Library**: 20+ built-in functions for math, strings, lists, and I/O
- **Enhanced LSP**: Semantic analysis, better diagnostics, context-aware completions
- **Native Functions**: Faster execution for built-in operations
- **Type Introspection**: `typeof()`, `toStr()`, `toInt()`, `toFloat()`, `toBool()`
- **Better Error Messages**: Runtime errors with line/column tracking

## Features

- **Gradual Typing**: Optional static types with Hindley-Milner inference
- **Hybrid Memory Model**: GC by default, ownership mode available
- **Modern Syntax**: Clean, expressive syntax inspired by multiple paradigms
- **Fast Compilation**: Rust-based compiler with LLVM backend
- **Rich Tooling**: REPL, IDE, formatter, linter, LSP server
- **Native Performance**: Compiles to native code via C or LLVM IR

## Installation

### From AUR (Arch Linux)

```bash
yay -S m5rcode
# or
paru -S m5rcode
```

### From Source (Arch Linux)

```bash
./scripts/bootstrap.sh
./scripts/build_arch.sh --install
```

### Manual Build

```bash
cargo build --release
sudo cp m5rcode /usr/local/bin/
sudo cp target/release/m5r* /usr/local/bin/
```

## Quick Start

```m5
# hello.m5
import std.io

fn main() {
    io.println("Hello, m5rcode!")
}
```

Run with:
```bash
m5rcode run hello.m5
```

Or start the REPL:
```bash
m5rcode repl
```

## Tools

- `m5rcode` - Unified CLI (run, repl, compile, fmt, lint, ide)
- `m5r` - Compiler
- `m5repl` - Interactive REPL
- `m5idle` - GTK-based IDE
- `m5fmt` - Code formatter
- `m5lint` - Linter
- `m5pkg` - Package manager
- `m5doc` - Documentation generator

## Documentation

- [Tutorial](docs/tutorial.md) - 20-minute introduction
- [API Reference](docs/api_reference.md) - Standard library documentation
- [Examples](docs/examples.md) - Code examples
- [Language Specification](SPEC/language_overview.md) - Complete language spec

## License

This project is dual-licensed under MIT OR Apache-2.0. See LICENSE file.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Security

Report security issues to security@m5rcode.org. See [SECURITY.md](SECURITY.md).
