mod solution;
mod utils;

use clap::Parser;
use solution::s01::calorie_counting::solution_day1_part1;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
}
