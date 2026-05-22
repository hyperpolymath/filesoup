# File Soup - Claude Code Instructions

This repository contains the File Soup file management utility.

## Project Structure

```
file-soup/
├── .claude/         # AI assistant instructions
├── .git/           # Version control
├── .gitignore      # Git ignore rules
├── .editorconfig   # Editor configuration
└── ...             # Utility files
```

## Build Commands

Refer to project-specific documentation.

## Coding Conventions

- Follow hyperpolymath standards
- All code must have SPDX license headers
- Use approved languages only (see CLAUDE.md)
- Document all non-obvious decisions

## Security

- No hardcoded secrets
- All secrets through environment variables or secret management
- SHA-pinned dependencies where applicable
- HTTPS only, no HTTP URLs
- No MD5/SHA1 for security purposes
