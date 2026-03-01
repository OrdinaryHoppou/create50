use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    New(NewArgs),

    /// Initialize the current working directory
    Init(InitArgs),
}

#[derive(Args)]
pub struct NewArgs {
    /// Programming language
    #[arg(value_enum, ignore_case = true)]
    pub language: Language,

    /// Project name
    pub name: String,

    // Initialize VCS
    #[arg(long, default_value = "true")]
    pub vcs: bool,
}

#[derive(Args)]
pub struct InitArgs {
    /// Programming language
    #[arg(value_enum, ignore_case = true)]
    pub language: Language,

    /// Initialize VCS
    #[arg(long, default_value = "true")]
    pub vcs: bool,
}

#[derive(Clone, ValueEnum)]
pub enum Language {
    C,
    Python,
}
