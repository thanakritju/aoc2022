use regex::Regex;

use crate::utils::load_file::load_file_split_two_lines;

pub fn solution_day5_part1(path: std::path::PathBuf) -> String {
    let input = load_file_split_two_lines(path);
    let (mut stack, commands) = parse_input(input);
    for (times, source, target) in commands {
        let mut i = 0;
        while i < times {
            let popped = stack[source].pop().expect("Not found");
            stack[target].push(popped);
            i += 1
        }
    }
    get_answer(stack)
}

pub fn solution_day5_part2(path: std::path::PathBuf) -> String {
    let input = load_file_split_two_lines(path);
    let (mut stack, commands) = parse_input(input);
    for (times, source, target) in commands {
        let num = stack[source].len() - times;
        let drained: Vec<char> = stack[source].drain(num..).collect();
        stack[target].extend(drained);
    }
    get_answer(stack)
}

fn get_answer(stack: [Vec<char>; 10]) -> String {
    let mut ans = "".to_string();
    for i in 1..10 {
        match stack[i].last() {
            Some(c) => ans = format!("{}{}", ans, c),
            None => {}
        };
    }
    ans.to_string()
}

fn parse_input(lines: Vec<String>) -> ([Vec<char>; 10], Vec<(usize, usize, usize)>) {
    let start_stack = generate_start_stack(lines.get(0).expect("No data").to_string());
    let commands: Vec<(usize, usize, usize)> = lines
        .get(1)
        .expect("No data")
        .lines()
        .map(parse_command)
        .collect();
    (start_stack, commands)
}

fn generate_start_stack(s: String) -> [Vec<char>; 10] {
    let mut start_stack: [Vec<char>; 10] = Default::default();
    let vec: Vec<String> = s.lines().map(|s| String::from(s)).rev().collect();
    for line in vec {
        for i in 1..10 {
            match line.as_bytes().get((i * 4) - 3) {
                Some(b) => {
                    let char = *b as char;
                    if char != ' ' {
                        start_stack[i].push(char)
                    }
                }
                None => {}
            }
        }
    }
    start_stack
}

fn parse_command(s: &str) -> (usize, usize, usize) {
    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let caps = re.captures(s).unwrap();
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
    fn test_parse_command() {
        assert_eq!(parse_command("move 16 from 4 to 9"), (16, 4, 9));
        assert_eq!(parse_command("move 7 from 7 to 6"), (7, 7, 6));
    }

    #[test]
    fn test_generate_start_stack() {
        let vec = generate_start_stack(
            "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 "
                .to_string(),
        );
        assert_eq!(vec[1].last(), Some(&'N'));
        assert_eq!(vec[2].last(), Some(&'D'));
        assert_eq!(vec[3].last(), Some(&'P'));
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day5_part1(PathBuf::from("src/solution/s05/example.txt")),
            "CMZ"
        );
        assert_eq!(
            solution_day5_part1(PathBuf::from("src/solution/s05/input.txt")),
            "QNNTGTPFN"
        );
        assert_eq!(
            solution_day5_part2(PathBuf::from("src/solution/s05/example.txt")),
            "MCD"
        );
        assert_eq!(
            solution_day5_part2(PathBuf::from("src/solution/s05/input.txt")),
            "GGNPJBTTR"
        );
    }
}
