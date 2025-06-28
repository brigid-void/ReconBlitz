use crate::scanner::{self, ScanResult};
use crate::ScanProfile;
use anyhow::Result;
use futures::future::join_all;
use std::collections::HashMap;
use std::time::Instant;

pub async fn run_scan(profile: &ScanProfile, target: &str, stealth: bool, benchmark: bool) -> Result<HashMap<String, ScanResult>> {
    let mut tool_futures = Vec::new();

    for tool in &profile.tools {
        let tool_clone = tool.clone();
        let target_clone = target.to_string();
        let future = async move {
            let start = Instant::now();
            let args = vec![target_clone.as_str()];
            let result = scanner::run_tool(&tool_clone, &args, stealth).await;
            
            if benchmark {
                let duration = start.elapsed();
                println!("Tool {} took {:?}", tool_clone, duration);
            }
            
            (tool_clone, result)
        };
        tool_futures.push(future);
    }

    let results: HashMap<String, ScanResult> = join_all(tool_futures).await.into_iter().collect();
    Ok(results)
}
