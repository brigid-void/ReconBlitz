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
