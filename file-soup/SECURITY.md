# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**DO NOT** create public GitHub issues for security vulnerabilities.

### Reporting Process

1. **Email**: Send details to security@fslint.org (or file maintainer email)
2. **Encrypted Communication**: Use PGP key if available (see .well-known/security.txt)
3. **Response Time**: You will receive acknowledgment within 48 hours
4. **Disclosure Timeline**: We aim to release fixes within 90 days

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Security Contact

See `.well-known/security.txt` for up-to-date contact information per RFC 9116.

## Security Best Practices

### For Users

1. **Keep Updated**: Always use the latest version
2. **Verify Downloads**: Check signatures and checksums
3. **Review Configs**: Audit plugin configurations before use
4. **Limit Permissions**: Run with minimal required privileges
5. **Hidden Files**: Use `--include-hidden` flag carefully

### For Plugin Developers

1. **Input Validation**: Validate all file paths and content
2. **No Arbitrary Execution**: Never execute arbitrary commands
3. **Resource Limits**: Implement timeouts and size limits
4. **Safe Dependencies**: Audit all dependencies
5. **Error Handling**: Use Result types, avoid panics

## Known Security Considerations

### Secret Scanner Plugin

- **Risk**: May detect false positives
- **Mitigation**: Review findings before taking action
- **Privacy**: Scan results stay local (offline-first)

### File System Access

- **Risk**: FSLint reads file metadata and content
- **Mitigation**: System directory protection, hidden file warnings
- **Permissions**: Respects OS-level file permissions

### Plugin System

- **Risk**: Plugins execute code in your process
- **Mitigation**: Only use trusted plugins, review source code
- **Future**: WASM sandboxing planned for v0.2.0

## Security Features

### Built-in Protections

1. **System Directory Protection**: Refuses to scan /system, /windows, etc.
2. **Hidden File Warnings**: Alerts when ratio of hidden:visible files is suspicious
3. **Path Sanitization**: Prevents path traversal attacks in output
4. **Memory Safety**: Rust's ownership system prevents buffer overflows
5. **Offline-First**: No network calls, works air-gapped

### Audit Trail

- All scans log to stderr (can be captured)
- Configuration changes tracked in config file
- Git integration shows file modification history

## Vulnerability Disclosure Policy

We follow **responsible disclosure**:

1. **Report received**: Acknowledge within 48 hours
2. **Triage**: Assess severity within 7 days
3. **Fix development**: Coordinate with reporter
4. **Security advisory**: Publish when fix is ready
5. **Credit**: Reporter credited in CHANGELOG and advisory

## Security Hall of Fame

Contributors who responsibly disclose vulnerabilities will be listed here.

*None yet - be the first!*

## Security Audits

| Date | Auditor | Scope | Findings |
|------|---------|-------|----------|
| 2025-11-22 | Self-audit | Initial release | 0 critical, 0 high |

## Compliance

- **RFC 9116**: security.txt in `.well-known/`
- **CWE Top 25**: Mitigated by Rust's memory safety
- **OWASP Top 10**: Input validation, no injection vectors

## Contact

- **Security Email**: See `.well-known/security.txt`
- **PGP Key**: See `.well-known/security.txt`
- **Response Time**: 48 hours acknowledgment, 7 days triage

---

Last updated: 2025-11-22
