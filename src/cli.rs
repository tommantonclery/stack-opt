use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "stack-opt")]
#[command(version = "0.1.0")]
#[command(about = "Analyze and optimize your project’s dependency stack", long_about = None)]
pub struct Args {
    /// Path to the project directory (default: current)
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// Skip analyzing devDependencies
    #[arg(long)]
    pub no_dev: bool,

    /// Output results as JSON
    #[arg(long)]
    pub json: bool,

    /// Exit with non-zero code if warnings or critical issues are found
    #[arg(long, value_enum, default_value_t = FailLevel::None)]
    pub fail_on: FailLevel,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum FailLevel {
    None,
    Warn,
    Crit,
}

pub fn parse_args() -> Args {
    Args::parse()
}
