mod solution;
mod utils;

use clap::Parser;
use solution::s01::calorie_counting::solution_day1_part1;
use solution::s01::calorie_counting::solution_day1_part2;
use solution::s02::rps::solution_day2_part1;
use solution::s02::rps::solution_day2_part2;
use solution::s03::reorganize::solution_day3_part1;
use solution::s03::reorganize::solution_day3_part2;
use solution::s04::campcleanup::solution_day4_part1;
use solution::s04::campcleanup::solution_day4_part2;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    match args.pattern.as_str() {
        "day1part1" => {
            println!("{}", solution_day1_part1(args.path))
        }
        "day1part2" => {
            println!("{}", solution_day1_part2(args.path))
        }
        "day2part1" => {
            println!("{}", solution_day2_part1(args.path))
        }
        "day2part2" => {
            println!("{}", solution_day2_part2(args.path))
        }
        "day3part1" => {
            println!("{}", solution_day3_part1(args.path))
        }
        "day3part2" => {
            println!("{}", solution_day3_part2(args.path))
        }
        "day4part1" => {
            println!("{}", solution_day4_part1(args.path))
        }
        "day4part2" => {
            println!("{}", solution_day4_part2(args.path))
        }
        _ => println!("In valid pattern"),
    }
}
