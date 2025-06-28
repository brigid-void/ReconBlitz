# Scanning Flow

This document describes the scanning flow in ReconBlitz, including the new hybrid scanning approach using RustScan and nmap.

## Hybrid Scanning

For `fast` and `stealth` profiles, ReconBlitz uses a hybrid scanning approach that combines the speed of RustScan with the power of nmap.

1.  **Port Discovery**: RustScan is used to quickly identify open ports on the target.
2.  **Service and Version Detection**: The open ports identified by RustScan are then passed to nmap for detailed service and version detection.

This approach provides a significant performance improvement over a full nmap scan, while still providing detailed information about the target.

## Argument Sanitization

All user-provided input is sanitized using the `shell-words` crate to prevent command injection attacks.

## Error Handling

Error handling has been improved to provide more detailed information when a scan fails. The error messages now include the exit code and stderr output of the failed command.
