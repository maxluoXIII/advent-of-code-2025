use clap::Parser;

/// Advent of Code 2025
#[derive(Parser)]
#[command(about)]
pub struct Args {
    /// the input file to use
    pub input: String,
}
