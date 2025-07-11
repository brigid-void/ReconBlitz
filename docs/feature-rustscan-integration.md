# Technical Specification: RustScan Integration

## 1. Summary

This document outlines the plan to integrate `RustScan` into `ReconBlitz` to address the performance bottleneck caused by `nmap`'s slow port scanning. The proposed solution is a hybrid approach where `RustScan` is used for initial port discovery, and its results are then piped to `nmap` for detailed analysis.

## 2. Rationale

The primary goal of this integration is to significantly improve the speed of `ReconBlitz` and deliver on its promise of "flash instant quick results".

*   **Speed**: `RustScan` can scan all 65,535 ports in under 3 seconds, which is a massive improvement over `nmap`'s default scan times.
*   **Robustness**: By piping the results to `nmap`, we retain its powerful and reliable analysis capabilities for service and version detection, OS fingerprinting, and script scanning.
*   **Feasibility**: This approach is less disruptive than a full `nmap` replacement and leverages the strengths of both tools.

## 3. Implementation Details

The integration will be done within the existing `ReconBlitz` Rust codebase.

### 3.1. `RustScan` Execution

`RustScan` will be executed as a command-line process from within the `ReconBlitz` application. The `std::process::Command` module in Rust will be used to run `RustScan` with the appropriate arguments.

```rust
use std::process::Command;

fn run_rustscan(target: &str) -> Result<String, std::io::Error> {
    let output = Command::new("rustscan")
        .arg("-a")
        .arg(target)
        .arg("--")
        // We can add nmap arguments here
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "RustScan command failed"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

### 3.2. Output Parsing

`RustScan`'s output, which includes the open ports, will be captured and parsed. The output can be formatted as JSON for easier parsing. We will need to add a function to parse this JSON and extract the list of open ports.

### 3.3. Piping to `nmap`

`RustScan` has a built-in feature to pipe its results directly to `nmap`. We can leverage this by passing `nmap` commands as arguments to `RustScan`. This simplifies the integration, as we won't need to manually parse `RustScan`'s output and then construct a new `nmap` command.

Example:
`rustscan -a example.com -- -A -sV`

This command will run `RustScan` on `example.com` and then run `nmap` with the `-A` and `-sV` flags on the discovered ports.

## 4. Acceptance Criteria

The integration will be considered complete when:

*   `ReconBlitz` uses `RustScan` for port discovery when a "fast" or "stealth" profile is selected.
*   The open ports found by `RustScan` are correctly passed to `nmap` for detailed scanning.
*   The final report generated by `ReconBlitz` includes the results from both `RustScan` (open ports) and `nmap` (detailed analysis).
*   The overall scan time for profiles using `RustScan` is significantly reduced compared to the previous implementation.
*   The integration is covered by unit and integration tests.
