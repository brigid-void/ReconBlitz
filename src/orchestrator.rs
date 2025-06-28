use crate::scanner::{run_hybrid_scan, run_tool, ScanResult};
use crate::ScanProfile;

use tokio::sync::Semaphore;
use std::sync::Arc;
use log::info;

pub async fn run_scan(
    profile: &ScanProfile,
    target: &str,
    stealth: bool,
    _benchmark: bool, // benchmark is unused for now
    timeout: u64,
    max_concurrent_scans: usize,
) -> anyhow::Result<Vec<(String, ScanResult)>> {
    log::info!("Starting scan on target: {}", target);

    let semaphore = Arc::new(Semaphore::new(max_concurrent_scans));
    let mut results = Vec::new();
    let mut tasks = Vec::new();

    for tool in &profile.tools {
        log::info!("Starting tool: {}", tool);
        let permit = semaphore.clone().acquire_owned().await?;
        let tool = tool.clone();
        let target = target.to_string();
        let stealth = stealth;

        tasks.push(tokio::spawn(async move {
            let _permit = permit; // Held until task completes
            let result = match tool.as_str() {
                "sleep" => { // A mock tool for testing
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    ScanResult::Success("Slept for 1s".to_string())
                }
                "nmap" => {
                    // Use the new hybrid scan for "nmap"
                    run_hybrid_scan(&target, stealth, timeout).await
                }
                _ => {
                    // Use the old run_tool for other tools
                    match tool.as_str() {
                        "dnsenum" => run_tool(&tool, &[&target], stealth, timeout).await,
                        "gobuster" => {
                            let url = format!("http://{}", target);
                            let args = [
                                "dir",
                                "-u",
                                &url,
                                "-w",
                                "/usr/share/wordlists/dirb/common.txt",
                            ];
                            run_tool(&tool, &args, stealth, timeout).await
                        }
                        "nikto" => run_tool(&tool, &["-h", &target], stealth, timeout).await,
                        _ => run_tool(&tool, &[&target], stealth, timeout).await,
                    }
                }
            };
            log::info!("Completed tool: {}", tool);
            // The result of the hybrid scan should still be stored under the "nmap" key
            (tool, result)
        }));
    }

    for task in tasks {
        let (tool, result) = task.await?;
        results.push((tool, result));
    }

    log::info!("Scan completed for target: {}", target);
    Ok(results)
}
