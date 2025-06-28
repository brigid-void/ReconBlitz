use reconblitz::{orchestrator, ScanProfile};
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_concurrent_scans_are_limited() {
    // This profile uses a mock "sleep" tool that takes 1 second to run.
    let profile = ScanProfile {
        name: "concurrency_test".to_string(),
        tools: vec!["sleep".to_string(); 10], // 10 tasks that each take 1s
    };

    let target = "localhost";
    let stealth = false;
    let benchmark = false;
    let timeout = 5; // 5s timeout for each task
    let max_concurrent_scans = 2; // Limit to 2 concurrent scans

    let start = Instant::now();
    let results = orchestrator::run_scan(
        &profile,
        target,
        stealth,
        benchmark,
        timeout,
        max_concurrent_scans,
    )
    .await
    .unwrap();

    let duration = start.elapsed();

    // 1. Verify that all tasks completed.
    assert_eq!(results.len(), 10, "Should have results from all 10 tasks");

    // 2. Verify that concurrency was limited.
    // With 10 tasks taking 1s each and a concurrency limit of 2, the total time
    // should be approximately 10 / 2 = 5 seconds.
    // We'll check if the duration is between 4.5 and 6 seconds to allow for some overhead.
    assert!(
        duration >= Duration::from_millis(4500),
        "Execution time should be at least 4.5s, but was {:?}",
        duration
    );
    assert!(
        duration <= Duration::from_millis(6000),
        "Execution time should be less than 6s, but was {:?}",
        duration
    );

    println!("Concurrency test passed in {:?}", duration);
}
