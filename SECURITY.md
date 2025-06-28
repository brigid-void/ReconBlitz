# Security Overview

ReconBlitz implements multiple security layers to protect users and systems:

## Input Security
- **Strict Validation**: All user inputs are validated against strict patterns
- **Sanitization**: Special characters are removed from target inputs
- **Parameterization**: Commands use argument arrays to prevent injection

## Runtime Security
- **Least Privilege**: Runs scans with minimal required permissions
- **Resource Limits**: Timeouts and concurrency controls prevent resource exhaustion
- **Isolation**: Docker containers provide process and filesystem isolation
- **Command Timeouts**: All scan commands have configurable timeouts (default 5 minutes) to prevent hanging processes

## Supply Chain Security
- **Dependency Scanning**: cargo-audit checks for vulnerable dependencies
- **Minimal Base Image**: Uses slim Debian image with only required packages
- **Immutable Infrastructure**: Containers are built from scratch for each scan
- **Non-Root Execution**: Containers run as non-root 'reconblitz' user

## Security Reporting
Please report security issues to security@example.com
