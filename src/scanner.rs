use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use log;

#[derive(Debug)]
pub enum ScanResult {
    Success(String),
    Failure(String),
    Error(String),
    Timeout,
}

pub async fn run_tool(tool: &str, args: &[&str], stealth: bool, timeout_secs: u64) -> ScanResult {
    log::info!("Running tool: {} with args: {:?}", tool, args);
    println!("[⏳] Running {}... (Press CTRL+C to skip)", tool);

    let command_future = async {
        let mut command = Command::new(tool);

        // Apply stealth options for supported tools
        if stealth {
            if tool == "nmap" {
                command
                    .arg("-T2")
                    .arg("--max-rtt-timeout")
                    .arg("500ms")
                    .arg("--scan-delay")
                    .arg("5s");
            }
        }

        for arg in args {
            command.arg(arg);
        }

        let output = command.output().await;

        match output {
            Ok(output) => {
                if output.status.success() {
                    ScanResult::Success(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    ScanResult::Failure(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => ScanResult::Error(e.to_string()),
        }
    };

    match timeout(Duration::from_secs(timeout_secs), command_future).await {
        Ok(result) => result,
        Err(_) => {
            println!("[⏱️] Skipping {} - timeout ({}s) reached", tool, timeout_secs);
            ScanResult::Timeout
        }
    }
}

pub async fn run_hybrid_scan(target: &str, stealth: bool, timeout_secs: u64) -> ScanResult {
    log::info!("Starting hybrid scan on: {}", target);
    println!("[⏳] Running RustScan + Nmap hybrid scan...");

    let mut command = Command::new("rustscan");
    command.arg("-a").arg(target).arg("--");

    if stealth {
        command
            .arg("-T2")
            .arg("--max-rtt-timeout")
            .arg("500ms")
            .arg("--scan-delay")
            .arg("5s");
    } else {
        command.arg("-A"); // -A is aggressive, includes -sV, -sC, and more.
    }

    // Use the provided timeout for the combined scan
    let command_future = command.output();
    match timeout(Duration::from_secs(timeout_secs), command_future).await {
        Ok(Ok(output)) => {
            if output.status.success() {
                ScanResult::Success(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                ScanResult::Failure(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Ok(Err(e)) => ScanResult::Error(format!(
            "Failed to execute rustscan: {}. Is it installed and in your PATH?",
            e
        )),
        Err(_) => {
            println!("[⏱️] Skipping Hybrid Scan - timeout ({}s) reached", timeout_secs);
            ScanResult::Timeout
        }
    }
}

pub async fn rustscan_scan(target: &str, ports: &str, timeout: u64) -> ScanResult {
    let output = Command::new("rustscan")
        .args(&["-a", target, "--ports", ports, "--timeout", &timeout.to_string()])
        .output()
        .await;

    match output {
        Ok(output) => {
            if output.status.success() {
                ScanResult::Success(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                ScanResult::Failure(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => ScanResult::Error(e.to_string()),
    }
}
