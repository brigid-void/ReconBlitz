use clap::{Parser, ValueEnum};
use reconblitz::{load_profiles, run, ScanProfile};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
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

    run(selected_profile, &args.target, format_str, args.stealth, args.benchmark).await
}
