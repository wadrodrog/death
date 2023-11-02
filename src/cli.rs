use std::path::PathBuf;

use clap::Parser;

/// A program that predicts your death date
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Your name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Your birthday
    #[arg(short, long)]
    pub birthday: Option<String>,

    /// Custom death reasons file
    #[arg(short, long, value_name = "FILE")]
    pub death_reasons: Option<PathBuf>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
