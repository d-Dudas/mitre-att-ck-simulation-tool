use clap::Parser;

/// MITRE ATT&CK Simulation Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args
{
    /// Technique ID (e.g., T1087)
    #[arg(short, long)]
    pub technique: String,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// results directory for results
    #[arg(short, long, default_value = "results")]
    pub results_dir: String,
}
