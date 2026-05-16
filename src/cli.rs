use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[command(name = "lakehouse-contracts-rs")]
#[command(about = "Validate metadata-driven lakehouse ingestion configs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Validate {
        config_path: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
pub enum OutputFormat {
    Human,
    Json,
    Markdown,
}
