# m5rcode Language Overview

## Design Goals

m5rcode aims to combine the best features of multiple programming paradigms:

- **Python**: Clean syntax, developer productivity, dynamic typing option
- **C**: Low-level control, performance, direct memory access
- **Java**: Strong typing option, class-based OOP, interfaces
- **HolyC**: Simplicity, JIT compilation, systems programming
- **Rust**: Memory safety, ownership model, zero-cost abstractions
- **Ruby**: Expressive syntax, blocks, metaprogramming

### Core Principles

1. **Gradual Typing**: Start dynamic, add types as needed
2. **Hybrid Memory Management**: GC by default, ownership when needed
3. **Native Performance**: Compile to native code via LLVM or C
4. **Developer Ergonomics**: Minimize boilerplate, maximize clarity
5. **Systems Programming**: Low-level access when required

## Developer Mental Model

### Type System

m5rcode uses **gradual typing** with Hindley-Milner type inference:

```m5
# Dynamic typing (inferred)
x = 42
name = "Alice"

# Static typing (explicit)
age: int = 30
price: float = 19.99

# Type inference with constraints
fn add(a, b) {
    return a + b  # Inferred: fn<T: Add>(T, T) -> T
}

# Explicit generic types
fn max<T: Ord>(a: T, b: T) -> T {
    return a > b ? a : b
}
```

### Memory Management

**Default: Garbage Collection**

```m5
# Automatic memory management
obj = MyClass()
list = [1, 2, 3]
# GC handles cleanup
```

**Optional: Ownership Mode**

```m5
# Explicit ownership (Rust-style)
own buffer = Buffer.new(1024)
# buffer moved, not copied
other = move buffer
# buffer no longer accessible
```

### Syntax Flexibility

m5rcode supports both indentation-based (Python) and brace-based (C) syntax:

```m5
# Indentation-based
fn factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

# Brace-based
fn factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

## Design Trade-offs

### 1. GC vs Ownership

**Decision**: GC by default, ownership opt-in

**Rationale**:
- Most applications benefit from GC simplicity
- Performance-critical code can use ownership
- Gradual migration path from prototype to production

**Alternative Considered**: Ownership-only (like Rust)
- **Rejected**: Too steep learning curve for beginners
- **Rejected**: Slows prototyping and experimentation

### 2. Indentation vs Braces

**Decision**: Support both syntaxes

**Rationale**:
- Python developers prefer indentation
- C/Java developers prefer braces
- Parser can handle both unambiguously

**Alternative Considered**: Indentation-only (like Python)
- **Rejected**: Alienates C/Java developers
- **Rejected**: Braces useful for one-liners

**Alternative Considered**: Braces-only (like C)
- **Rejected**: More verbose for simple code
- **Rejected**: Loses Python's visual clarity

### 3. Dynamic vs Static Typing

**Decision**: Gradual typing (both supported)

**Rationale**:
- Dynamic typing for rapid prototyping
- Static typing for production code
- Type inference reduces annotation burden

**Alternative Considered**: Static-only (like Java)
- **Rejected**: Too rigid for scripting use cases
- **Rejected**: Requires upfront design

**Alternative Considered**: Dynamic-only (like Python)
- **Rejected**: No compile-time safety guarantees
- **Rejected**: Poor IDE support for large codebases

### 4. Implementation Language: Rust

**Decision**: Implement compiler in Rust

**Rationale**:
- Memory safety without GC overhead
- Excellent LLVM bindings
- Strong type system for compiler correctness
- Growing ecosystem of parsing libraries

**Alternative Considered**: C++
- **Rejected**: Memory safety issues
- **Rejected**: Slower development cycle

**Alternative Considered**: Self-hosting (m5rcode)
- **Rejected**: Chicken-and-egg problem
- **Rejected**: Immature tooling initially

### 5. Target Platform: Arch Linux First

**Decision**: Prioritize Arch Linux packaging

**Rationale**:
- Rolling release = latest dependencies
- AUR provides easy distribution
- Strong developer community
- Excellent documentation

**Alternative Considered**: Ubuntu/Debian first
- **Rejected**: Outdated packages in stable releases
- **Rejected**: PPA maintenance overhead

**Alternative Considered**: Platform-agnostic
- **Rejected**: Dilutes initial focus
- **Rejected**: Harder to test thoroughly

## Language Features

### Core Features (v1.0)

- Variables and basic types (int, float, string, bool)
- Functions (first-class, closures)
- Control flow (if/else, while, for, match)
- Classes and objects
- Modules and imports
- Basic standard library (IO, collections, math)

### Advanced Features (v1.1+)

- Generics and traits
- Async/await
- Pattern matching
- Operator overloading
- Metaprogramming
- FFI (C interop)
- Ownership mode

## Compilation Model

```
.m5 source → Lexer → Parser → AST → Type Checker → IR → LLVM/C → Native Binary
                                                    ↓
                                                Interpreter (REPL)
```

### Execution Modes

1. **Interpreted**: REPL and scripting
2. **JIT**: Fast startup for development
3. **AOT**: Optimized native binaries for production

## Standard Library Design

Organized into modules:

- `std.io` - Input/output
- `std.collections` - Data structures
- `std.math` - Mathematical functions
- `std.string` - String manipulation
- `std.fs` - File system operations
- `std.net` - Networking
- `std.async` - Asynchronous programming
- `std.ffi` - Foreign function interface

## Tooling Philosophy

- **m5r**: Compiler (like rustc)
- **m5repl**: Interactive REPL (like python)
- **m5idle**: Integrated IDE (like IDLE)
- **m5fmt**: Code formatter (like rustfmt)
- **m5lint**: Linter (like clippy)
- **m5pkg**: Package manager (like cargo)
- **m5doc**: Documentation generator (like rustdoc)

## Community and Ecosystem

- Open source (MIT/Apache-2.0)
- Community-driven development
- Package registry for sharing libraries
- LSP support for editor integration
- Comprehensive documentation
