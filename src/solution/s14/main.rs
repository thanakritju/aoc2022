use std::{num::ParseIntError, str::FromStr};

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day14_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let coordinates = parse_input(input);
    0
}

pub fn solution_day14_part2(path: std::path::PathBuf) -> i32 {
    0
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Coordinate>> {
    input
        .into_iter()
        .map(|line| {
            line.split(" -> ")
                .map(|s| Coordinate::from_str(s).unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct Coordinate(usize, usize);

impl FromStr for Coordinate {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').unwrap();

        let x_fromstr = x.parse::<usize>()?;
        let y_fromstr = y.parse::<usize>()?;

        Ok(Coordinate(x_fromstr, y_fromstr))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse() {
        let out: Coordinate = "498,6".parse().expect("can't parse");
        assert_eq!(Coordinate(498, 6), out);
        assert_eq!(Coordinate(503, 4), Coordinate::from_str("503,4").unwrap());
    }

    #[test]
    fn test_solution() {
        // assert_eq!(
        //     solution_day14_part1(PathBuf::from("src/solution/s14/example.txt")),
        //     0
        // );
        assert_eq!(
            solution_day14_part1(PathBuf::from("src/solution/s14/input.txt")),
            0
        );
        assert_eq!(
            solution_day14_part2(PathBuf::from("src/solution/s14/example.txt")),
            0
        );
        assert_eq!(
            solution_day14_part2(PathBuf::from("src/solution/s14/input.txt")),
            0
        );
    }
}
