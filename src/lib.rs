use std::fs;
use std::time::Instant;

pub mod scanner;
pub mod orchestrator;
pub mod reporter;

pub async fn run(profile: &ScanProfile, target: &str, format: &str, stealth: bool, benchmark: bool) -> anyhow::Result<()> {
    let start = Instant::now();
    let results = orchestrator::run_scan(profile, target, stealth, benchmark).await?;

    let report = match format {
        "html" => reporter::generate_html_report(&results),
        "json" => reporter::generate_json_report(&results),
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
    };

    if benchmark {
        let duration = start.elapsed();
        println!("Benchmark: Scan completed in {:?}", duration);
    }

    if format == "html" {
        fs::write("report.html", &report)?;
        println!("HTML report saved to report.html");
    } else {
        println!("{}", report);
    }

    Ok(())
}

pub struct ScanProfile {
    pub name: String,
    pub tools: Vec<String>,
}

impl Clone for ScanProfile {
    fn clone(&self) -> Self {
        ScanProfile { 
            name: self.name.clone(), 
            tools: self.tools.clone(),
        }
    }
}

pub fn load_profiles() -> Vec<ScanProfile> {
    vec![
        ScanProfile {
            name: "fast".to_string(),
            tools: vec!["nmap".to_string(), "dnsenum".to_string()],
        },
        ScanProfile {
            name: "full".to_string(),
            tools: vec![
                "nmap".to_string(),
                "dnsenum".to_string(),
                "gobuster".to_string(),
                "nikto".to_string(),
            ],
        },
    ]
}
