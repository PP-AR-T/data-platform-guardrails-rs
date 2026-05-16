mod cli;
mod config;
mod report;
mod validation;
mod validators;

use anyhow::Result;
use clap::Parser;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Commands::Validate {
            config_path,
            format,
        } => {
            let cfg = config::load_config(&config_path)?;
            let report = validation::validate_config(&cfg);
            let output = report::render_output(&report, &format, &config_path);
            println!("{output}");
            let exit_code = if report.has_failures() { 1 } else { 0 };
            std::process::exit(exit_code);
        }
    }
}
