use std::collections::{HashSet, VecDeque};

use crate::utils::load_file::load_file_to_string;

pub fn solution_day6_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string(path);
    get_first_marker(input, 4)
}

pub fn solution_day6_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string(path);
    get_first_marker(input, 14)
}

fn get_first_marker(stream: String, max: i32) -> i32 {
    let mut window: VecDeque<char> = VecDeque::new();
    for (i, c) in stream.chars().enumerate() {
        if window.len() == max.try_into().unwrap() {
            if is_all_unique(&window) {
                return i.try_into().unwrap();
            } else {
                window.pop_front();
                window.push_back(c);
            }
        } else {
            window.push_back(c);
        }
    }
    0
}

fn is_all_unique(chars: &VecDeque<char>) -> bool {
    let l = chars.len();
    let mut set_chars = HashSet::new();
    for c in chars {
        set_chars.insert(c);
    }
    set_chars.len() == l
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_get_first_marker() {
        assert_eq!(
            get_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4),
            7
        );
        assert_eq!(
            get_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
            23
        );
        assert_eq!(
            get_first_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4),
            6
        );
        assert_eq!(
            get_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),
            29
        );
        assert_eq!(
            get_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),
            11
        );
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day6_part1(PathBuf::from("src/solution/s06/example.txt")),
            11
        );
        assert_eq!(
            solution_day6_part1(PathBuf::from("src/solution/s06/input.txt")),
            1175
        );
        assert_eq!(
            solution_day6_part2(PathBuf::from("src/solution/s06/example.txt")),
            26
        );
        assert_eq!(
            solution_day6_part2(PathBuf::from("src/solution/s06/input.txt")),
            3217
        );
    }
}
