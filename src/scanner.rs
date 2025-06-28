use tokio::process::Command;
use std::time::Duration;
use tokio::time::timeout;

pub enum ScanResult {
    Success(String),
    Failure(String),
    Error(String),
    Timeout,
}

pub async fn run_tool(tool: &str, args: &[&str], stealth: bool) -> ScanResult {
    println!("[⏳] Running {}... (Press CTRL+C to skip)", tool);
    
    let command_future = async {
        let mut command = Command::new(tool);
        
        // Apply stealth options for supported tools
        if stealth {
            match tool {
                "nmap" => {
                    command.arg("-T2")
                        .arg("--max-rtt-timeout")
                        .arg("500ms")
                        .arg("--scan-delay")
                        .arg("5s");
                }
                _ => {}
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
    
    match timeout(Duration::from_secs(30), command_future).await {
        Ok(result) => result,
        Err(_) => {
            println!("[⏱️] Skipping {} - timeout (30s) reached", tool);
            ScanResult::Timeout
        }
    }
}

/// Runs RustScan for fast port discovery and pipes results to nmap for detailed analysis
pub async fn run_rustscan_with_nmap(target: &str, stealth: bool) -> ScanResult {
    println!("[⚡] Running RustScan with Nmap... (Press CTRL+C to skip)");
    
    let command_future = async {
        let mut command = Command::new("rustscan");
        command.arg("-a").arg(target).arg("--");
        
        // Add nmap arguments
        if stealth {
            command
                .arg("-T2")
                .arg("--max-rtt-timeout")
                .arg("500ms")
                .arg("--scan-delay")
                .arg("5s");
        }
        command.arg("-A").arg("-sV");
        
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
    
    // Use a longer timeout (60s) since this is a full scan
    match timeout(Duration::from_secs(60), command_future).await {
        Ok(result) => result,
        Err(_) => {
            println!("[⏱️] Skipping RustScan with Nmap - timeout (60s) reached");
            ScanResult::Timeout
        }
    }
}
