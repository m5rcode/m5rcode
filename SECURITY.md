# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@m5rcode.org**

Include the following information:

- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the vulnerability
- Suggested fix (if any)

## Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-30 days
  - Medium: 30-90 days
  - Low: Best effort

## Disclosure Policy

- Security issues are fixed privately
- Fixes are released as soon as possible
- CVE IDs are requested for significant vulnerabilities
- Public disclosure occurs after fix is released
- Credit given to reporters (unless anonymity requested)

## Security Considerations

### Compiler Security

- **Input Validation**: All source code is untrusted input
- **Resource Limits**: Parser has recursion depth limits
- **Memory Safety**: Rust prevents most memory vulnerabilities
- **Injection**: No code injection in compiler itself

### Runtime Security

- **Memory Safety**: GC prevents use-after-free
- **Sandboxing**: No built-in sandboxing (use OS-level)
- **FFI**: Foreign function calls are unsafe by design
- **File Access**: No restrictions on file system access

### Known Limitations

1. **No Sandboxing**: m5rcode programs run with full user privileges
2. **FFI Unsafe**: C interop can cause memory corruption
3. **Resource Exhaustion**: No built-in limits on memory/CPU
4. **Code Injection**: `eval()` not implemented (by design)

### Best Practices

#### For m5rcode Users

- Don't run untrusted m5rcode programs
- Use OS-level sandboxing (containers, VMs)
- Validate all external input
- Use ownership mode for security-critical code
- Audit FFI calls carefully

#### For m5rcode Developers

- Validate all parser inputs
- Add bounds checking
- Use safe Rust practices
- Fuzz test parsers and interpreters
- Review FFI code carefully

## Security Features

### Current

- Memory-safe compiler (Rust)
- Bounds-checked arrays (in safe mode)
- Type safety (when types specified)
- No buffer overflows in compiler

### Planned

- Capability-based security
- Resource limits (memory, CPU)
- Sandboxed execution mode
- Secure FFI wrappers
- Static analysis tools

## Vulnerability Disclosure Examples

### Example 1: Parser Crash

```
Subject: Parser crash on deeply nested expressions

Description: The parser crashes with stack overflow on deeply nested
parentheses due to unbounded recursion.

Reproduction:
1. Create file with 10000 nested parentheses: (((((...))))
2. Run: m5r file.m5
3. Observe: Stack overflow crash

Impact: Denial of service

Suggested Fix: Add recursion depth limit in parser
```

### Example 2: Memory Leak

```
Subject: Memory leak in garbage collector

Description: Circular references are not collected, causing memory leak.

Reproduction:
1. Create two objects with references to each other
2. Remove external references
3. Observe: Memory not freed

Impact: Resource exhaustion

Suggested Fix: Implement cycle detection in GC
```

## Security Updates

Security updates are released as:
- Patch versions (0.1.x)
- Announced on GitHub Security Advisories
- Documented in CHANGELOG with [SECURITY] tag

## Contact

- Security Email: security@m5rcode.org
- PGP Key: (To be added)
- GitHub Security: https://github.com/m5rcode/m5rcode/security

## Acknowledgments

We thank the following researchers for responsible disclosure:

(List will be updated as vulnerabilities are reported and fixed)

## Additional Resources

- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [CWE Top 25](https://cwe.mitre.org/top25/)

---

Last Updated: 2026-02-28
