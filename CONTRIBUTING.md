# Contributing to m5rcode

Thank you for your interest in contributing to m5rcode! This document provides guidelines for contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/m5rcode.git`
3. Create a branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Test your changes
6. Commit: `git commit -m "Add my feature"`
7. Push: `git push origin feature/my-feature`
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+ and Cargo
- pkg-config
- GTK3 development files (for m5idle)
- LLVM (for code generation)

### Building

```bash
./scripts/bootstrap.sh
./scripts/build_arch.sh
```

### Running Tests

```bash
cd compiler
cargo test

cd ../runtime
cargo test
```

## Contribution Areas

### Compiler

- Lexer improvements
- Parser enhancements
- Type checker implementation
- Code generation (LLVM/C backend)
- Error messages

### Runtime

- Garbage collector optimization
- FFI implementation
- Standard library functions
- Performance improvements

### Tools

- REPL features (history, completion)
- IDE functionality
- Formatter rules
- Linter checks
- LSP features

### Documentation

- Tutorial improvements
- API documentation
- Examples
- Specification clarifications

### Testing

- Unit tests
- Integration tests
- Benchmarks
- Fuzzing

## Coding Standards

### Rust Code

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` and fix warnings
- Add doc comments (`///`) for public items
- Write tests for new functionality

### m5rcode Code

- Use 4-space indentation
- Follow naming conventions:
  - `snake_case` for variables and functions
  - `PascalCase` for classes and types
  - `UPPER_CASE` for constants
- Add comments for complex logic
- Write examples in documentation

## Pull Request Process

1. **Update documentation** - Add/update docs for new features
2. **Add tests** - Include tests for bug fixes and features
3. **Run tests** - Ensure all tests pass
4. **Update CHANGELOG** - Add entry describing changes
5. **One feature per PR** - Keep PRs focused
6. **Descriptive commits** - Write clear commit messages

### Commit Message Format

```
<type>: <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Build/tooling changes

Example:
```
feat: Add pattern matching to parser

Implement pattern matching syntax in the parser including:
- Match expressions
- Pattern types (literal, wildcard, destructuring)
- Exhaustiveness checking

Closes #123
```

## Testing Guidelines

### Unit Tests

Test individual components in isolation:

```rust
#[test]
fn test_lexer_tokenizes_integers() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].typ, TokenType::Int(42));
}
```

### Integration Tests

Test end-to-end functionality:

```m5
test("fibonacci works", || {
    result = fibonacci(10)
    test.assert_eq(result, 55, "fib(10) should be 55")
})
```

## Documentation Guidelines

- Use clear, concise language
- Provide code examples
- Explain "why" not just "what"
- Keep examples runnable
- Update API docs when changing interfaces

## Issue Reporting

### Bug Reports

Include:
- m5rcode version
- Operating system
- Steps to reproduce
- Expected behavior
- Actual behavior
- Error messages
- Minimal code example

### Feature Requests

Include:
- Use case description
- Proposed syntax/API
- Examples
- Alternatives considered
- Implementation ideas (optional)

## Review Process

1. Maintainer reviews PR
2. Feedback provided
3. Author addresses feedback
4. Maintainer approves
5. PR merged

## Release Process

1. Update version in `Cargo.toml` files
2. Update CHANGELOG
3. Create git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. Build release: `./scripts/make_release.sh`
6. Create GitHub release
7. Update AUR package

## Community

- GitHub Discussions: Ask questions, share ideas
- GitHub Issues: Report bugs, request features
- Pull Requests: Contribute code

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.

## Questions?

Open an issue or start a discussion on GitHub!

Thank you for contributing to m5rcode! 🚀
