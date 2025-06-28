use reconblitz::scanner::{run_hybrid_scan, ScanResult};

#[tokio::test]
async fn test_run_hybrid_scan_success() {
    // This test requires rustscan and nmap to be installed and in the PATH.
    // It also requires root privileges to run.
    // We will mock the a successful scan by checking for a known open port on localhost.
    let result = run_hybrid_scan("localhost", false, 60).await;
    match result {
        ScanResult::Success(output) => {
            assert!(output.contains("Nmap scan report for localhost"));
        }
        _ => panic!("Expected a successful scan"),
    }
}

#[tokio::test]
async fn test_run_hybrid_scan_stealth() {
    // This test requires rustscan and nmap to be installed and in the PATH.
    // It also requires root privileges to run.
    // We will mock the a successful scan by checking for a known open port on localhost.
    let result = run_hybrid_scan("localhost", true, 120).await;
    match result {
        ScanResult::Success(output) => {
            assert!(output.contains("Nmap scan report for localhost"));
        }
        _ => panic!("Expected a successful scan"),
    }
}

#[tokio::test]
async fn test_run_hybrid_scan_invalid_target() {
    // This test does not require root privileges.
    let result = run_hybrid_scan("invalid-target", false, 60).await;
    match result {
        ScanResult::Failure(_) => {
            // The scan is expected to fail for an invalid target.
            // We don't need to check the error message, just that it failed.
        }
        other => panic!("Expected a failed scan for an invalid target, but got {:?}", other),
    }
}
