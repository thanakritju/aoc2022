use std::{cell::RefCell, rc::Rc};

use crate::utils::load_file::load_file_split_two_lines;

pub fn solution_day13_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_split_two_lines(path);
    let items: Vec<[String; 2]> = input
        .iter()
        .map(|line| parse_line(line.to_owned()))
        .collect();

    0
}

pub fn solution_day13_part2(path: std::path::PathBuf) -> i32 {
    0
}

fn are_in_right_order(l1: String, l2: String) -> bool {
    false
}

pub fn parse_line(line: String) -> [String; 2] {
    let mut array = ["".to_string(), "".to_string()];
    let eachs: Vec<&str> = line.lines().collect();
    array[0] = eachs.get(0).expect("no data").to_string();
    array[1] = eachs.get(1).expect("no data").to_string();
    array
}

struct Node {
    children: Vec<Rc<RefCell<Node>>>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[ignore]
    #[test]
    fn test_are_in_right_order() {
        assert_eq!(
            are_in_right_order("[1,1,3,1,1]".to_string(), "[1,1,5,1,1]".to_string()),
            true
        );
        assert_eq!(
            are_in_right_order("[[1],[2,3,4]]".to_string(), "[[1],4]".to_string()),
            true
        );
        assert_eq!(
            are_in_right_order("[9]".to_string(), "[[8,7,6]]".to_string()),
            false
        );
        assert_eq!(
            are_in_right_order("[[4,4],4,4]".to_string(), "[[4,4],4,4,4]".to_string()),
            false
        );
        assert_eq!(
            are_in_right_order("[7,7,7,7]".to_string(), "[7,7,7]".to_string()),
            false
        );
        assert_eq!(
            are_in_right_order("[]".to_string(), "[3]".to_string()),
            true
        );
        assert_eq!(
            are_in_right_order("[[[]]]".to_string(), "[[]]".to_string()),
            false
        );
        assert_eq!(
            are_in_right_order(
                "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
                "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string()
            ),
            false
        );
    }

    #[ignore]
    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day13_part1(PathBuf::from("src/solution/s13/example.txt")),
            13
        );
        assert_eq!(
            solution_day13_part1(PathBuf::from("src/solution/s13/input.txt")),
            0
        );
        assert_eq!(
            solution_day13_part2(PathBuf::from("src/solution/s13/example.txt")),
            0
        );
        assert_eq!(
            solution_day13_part2(PathBuf::from("src/solution/s13/input.txt")),
            0
        );
    }
}
