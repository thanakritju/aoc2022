use regex::Regex;

use crate::utils::load_file::load_file_split_two_lines;

pub fn solution_day5_part1(path: std::path::PathBuf) -> String {
    let input = load_file_split_two_lines(path);
    "0".to_string()
}

pub fn solution_day5_part2(path: std::path::PathBuf) -> String {
    "0".to_string()
}

fn parse_input(lines: Vec<String>) -> ([Vec<char>; 9], Vec<(i8, i8, i8)>) {
    let start_stack = generate_start_stack(lines.get(0).expect("No data").to_string());
    let commands: Vec<(i8, i8, i8)> = lines
        .get(1)
        .expect("No data")
        .lines()
        .map(parse_command)
        .collect();
    (start_stack, commands)
}

fn generate_start_stack(str: String) -> [Vec<char>; 9] {
    let start_stack: [Vec<char>; 9] = Default::default();
    start_stack
}

fn parse_command(str: &str) -> (i8, i8, i8) {
    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let caps = re.captures(str).unwrap();
    let num1 = caps.get(1).map_or("0", |m| m.as_str());
    let num2 = caps.get(2).map_or("0", |m| m.as_str());
    let num3 = caps.get(3).map_or("0", |m| m.as_str());
    (
        num1.parse().expect("parse error"),
        num2.parse().expect("parse error"),
        num3.parse().expect("parse error"),
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day5_part1(PathBuf::from("src/solution/05/example.txt")),
            "CMZ"
        );
        assert_eq!(
            solution_day5_part1(PathBuf::from("src/solution/s05/input.txt")),
            "0"
        );
        assert_eq!(
            solution_day5_part2(PathBuf::from("src/solution/s05/example.txt")),
            "0"
        );
        assert_eq!(
            solution_day5_part2(PathBuf::from("src/solution/s05/input.txt")),
            "0"
        );
    }
}
