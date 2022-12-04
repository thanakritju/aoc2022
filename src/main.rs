mod solution;
mod utils;

use clap::Parser;
use solution::s01::main::*;
use solution::s02::main::*;
use solution::s03::main::*;
use solution::s04::main::*;
use solution::s05::main::*;
use solution::s06::main::*;
use solution::s07::main::*;
use solution::s08::main::*;
use solution::s09::main::*;
use solution::s10::main::*;
use solution::s11::main::*;
use solution::s12::main::*;
use solution::s13::main::*;
use solution::s14::main::*;
use solution::s15::main::*;
use solution::s16::main::*;
use solution::s17::main::*;
use solution::s18::main::*;
use solution::s19::main::*;
use solution::s20::main::*;
use solution::s21::main::*;
use solution::s22::main::*;
use solution::s23::main::*;
use solution::s24::main::*;
use solution::s25::main::*;

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
        "day5part1" => {
            println!("{}", solution_day5_part1(args.path))
        }
        "day5part2" => {
            println!("{}", solution_day5_part2(args.path))
        }
        "day6part1" => {
            println!("{}", solution_day6_part1(args.path))
        }
        "day6part2" => {
            println!("{}", solution_day6_part2(args.path))
        }
        "day7part1" => {
            println!("{}", solution_day7_part1(args.path))
        }
        "day7part2" => {
            println!("{}", solution_day7_part2(args.path))
        }
        "day8part1" => {
            println!("{}", solution_day8_part1(args.path))
        }
        "day8part2" => {
            println!("{}", solution_day8_part2(args.path))
        }
        "day9part1" => {
            println!("{}", solution_day9_part1(args.path))
        }
        "day9part2" => {
            println!("{}", solution_day9_part2(args.path))
        }
        "day10part1" => {
            println!("{}", solution_day10_part1(args.path))
        }
        "day10part2" => {
            println!("{}", solution_day10_part2(args.path))
        }
        "day11part1" => {
            println!("{}", solution_day11_part1(args.path))
        }
        "day11part2" => {
            println!("{}", solution_day11_part2(args.path))
        }
        "day12part1" => {
            println!("{}", solution_day12_part1(args.path))
        }
        "day12part2" => {
            println!("{}", solution_day12_part2(args.path))
        }
        "day13part1" => {
            println!("{}", solution_day13_part1(args.path))
        }
        "day13part2" => {
            println!("{}", solution_day13_part2(args.path))
        }
        "day14part1" => {
            println!("{}", solution_day14_part1(args.path))
        }
        "day14part2" => {
            println!("{}", solution_day14_part2(args.path))
        }
        "day15part1" => {
            println!("{}", solution_day15_part1(args.path))
        }
        "day15part2" => {
            println!("{}", solution_day15_part2(args.path))
        }
        "day16part1" => {
            println!("{}", solution_day16_part1(args.path))
        }
        "day16part2" => {
            println!("{}", solution_day16_part2(args.path))
        }
        "day17part1" => {
            println!("{}", solution_day17_part1(args.path))
        }
        "day17part2" => {
            println!("{}", solution_day17_part2(args.path))
        }
        "day18part1" => {
            println!("{}", solution_day18_part1(args.path))
        }
        "day18part2" => {
            println!("{}", solution_day18_part2(args.path))
        }
        "day19part1" => {
            println!("{}", solution_day19_part1(args.path))
        }
        "day19part2" => {
            println!("{}", solution_day19_part2(args.path))
        }
        "day20part1" => {
            println!("{}", solution_day20_part1(args.path))
        }
        "day20part2" => {
            println!("{}", solution_day20_part2(args.path))
        }
        "day21part1" => {
            println!("{}", solution_day21_part1(args.path))
        }
        "day21part2" => {
            println!("{}", solution_day21_part2(args.path))
        }
        "day22part1" => {
            println!("{}", solution_day22_part1(args.path))
        }
        "day22part2" => {
            println!("{}", solution_day22_part2(args.path))
        }
        "day23part1" => {
            println!("{}", solution_day23_part1(args.path))
        }
        "day23part2" => {
            println!("{}", solution_day23_part2(args.path))
        }
        "day24part1" => {
            println!("{}", solution_day24_part1(args.path))
        }
        "day24part2" => {
            println!("{}", solution_day24_part2(args.path))
        }
        "day25part1" => {
            println!("{}", solution_day25_part1(args.path))
        }
        "day25part2" => {
            println!("{}", solution_day25_part2(args.path))
        }
        _ => println!("In valid pattern"),
    }
}
