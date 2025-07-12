// src/main.rs
/*
 * Main executable for CipherFlow
 */

use clap::Parser;
use cipherflow::{Result, run};

#[derive(Parser)]
#[command(version, about = "CipherFlow - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
