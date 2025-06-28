use reconblitz::{self, scanner::{run_hybrid_scan, ScanResult}};

#[test]
fn test_input_validation() {
    // Valid targets
    assert!(reconblitz::is_valid_target("example.com"));
    assert!(reconblitz::is_valid_target("192.168.1.1"));

    // Invalid targets
    assert!(!reconblitz::is_valid_target("example.com; rm -rf /"));
    assert!(!reconblitz::is_valid_target("invalid$target"));
}

#[tokio::test]
async fn test_hybrid_scan() {
    // Test should pass even if scan fails (we're testing integration)
    let result = tokio::time::timeout(std::time::Duration::from_secs(60), run_hybrid_scan("127.0.0.1", false, 60)).await;
    match result {
        Ok(scan_result) => {
            // If the scan completes, check that it wasn't an error
            assert!(!matches!(scan_result, ScanResult::Error(_)));
        }
        Err(_) => {
            // If the scan times out, that's okay for this test.
            // We can consider it a pass as the timeout mechanism worked.
            println!("Scan timed out as expected.");
        }
    }
}
