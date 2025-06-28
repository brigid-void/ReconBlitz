use clap::{Parser, ValueEnum};
use reconblitz::{load_profiles, run};
use std::process;
use std::path::PathBuf;
use std::fs::File;
use log::{set_boxed_logger, set_max_level, LevelFilter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The target domain or IP address to scan
    #[arg(short, long)]
    target: String,

    /// The scan profile to use
    #[arg(short, long, value_enum, default_value_t = Profile::Fast)]
    profile: Profile,

    /// The output format for the report
    #[arg(long, value_enum, default_value_t = Format::Html)]
    format: Format,

    /// Enable stealth mode (slower scans to avoid detection)
    #[arg(short, long)]
    stealth: bool,

    /// Run in benchmark mode (measure and report scan times)
    #[arg(short, long)]
    benchmark: bool,

    /// Timeout in seconds for each scan command. Default: 300 seconds (5 minutes).
    #[arg(short = 't', long, default_value = "300")]
    timeout: u64,

    /// Maximum number of concurrent scans. Default: 5.
    #[arg(short = 'c', long, default_value = "5")]
    max_concurrent_scans: usize,

    /// Optional path to a log file. If provided, logs will be written to this file in addition to the console.
    #[arg(short = 'l', long)]
    log_file: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Profile {
    Fast,
    Full,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Html,
    Json,
}

fn is_valid_target(target: &str) -> bool {
    // Validate as IP address
    if target.parse::<std::net::IpAddr>().is_ok() {
        return true;
    }
    
    // Validate as domain name with allowlist
    // Allow only ASCII alphanumeric, hyphen, and period
    if target.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.') {
        // Additional checks for homograph prevention
        // 1. Check for mixed script (requires unicode-segmentation crate)
        // 2. Check for confusable characters (requires unicode-security crate)
        // For MVP, we'll do basic TLD validation
        let parts: Vec<&str> = target.split('.').collect();
        if parts.len() < 2 {
            return false; // Must have at least domain and TLD
        }
        
        // Validate each part
        return parts.iter().all(|part| {
            !part.is_empty() && 
            part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') &&
            !part.starts_with('-') && 
            !part.ends_with('-') &&
            part.len() <= 63
        });
    }
    
    false
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    if let Some(log_path) = &args.log_file {
        let log_file = File::create(log_path)?;
        let logger = Box::new(log_file);
        log::set_boxed_logger(logger)?;
        log::set_max_level(LevelFilter::Info);
    }

    // Validate target input
    if !is_valid_target(&args.target) {
        eprintln!("Error: Invalid target format. Must be valid IP or domain name");
        process::exit(1);
    }

    // Sanitize target input
    let sanitized_target = args.target.replace(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '.', "");

    let profiles = load_profiles();

    let profile_name = match args.profile {
        Profile::Fast => "fast",
        Profile::Full => "full",
    };

    let selected_profile = profiles
        .iter()
        .find(|p| p.name == profile_name)
        .expect("Profile not found. This is an internal error.");

    let format_str = match args.format {
        Format::Html => "html",
        Format::Json => "json",
    };

    // Pass timeout to scanner
    run(selected_profile, &sanitized_target, format_str, args.stealth, args.benchmark, args.timeout).await
}
