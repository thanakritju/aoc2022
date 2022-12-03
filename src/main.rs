mod solution;
mod utils;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
}
