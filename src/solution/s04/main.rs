use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day4_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    input
        .into_iter()
        .filter(|x| is_fully_contain(x.to_string()))
        .count() as i32
}

pub fn solution_day4_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    input
        .into_iter()
        .filter(|x| is_partly_contain(x.to_string()))
        .count() as i32
}

fn is_fully_contain(line: String) -> bool {
    let (x1, y1, x2, y2) = parse_input(line);
    let mut array: [i32; 100] = [0; 100];
    let mut i = x1;
    while i <= y1 {
        array[i as usize] += 1;
        i += 1;
    }
    let mut i = x2;
    while i <= y2 {
        array[i as usize] -= 1;
        i += 1;
    }
    array.into_iter().all(|e| e >= 0) || array.into_iter().all(|e| e <= 0)
}

fn is_partly_contain(line: String) -> bool {
    let (x1, y1, x2, y2) = parse_input(line);
    let mut array: [i32; 100] = [0; 100];
    let mut i = x1;
    while i <= y1 {
        array[i as usize] += 1;
        i += 1;
    }
    let mut i = x2;
    while i <= y2 {
        if array[i as usize] > 0 {
            return true;
        }
        i += 1;
    }
    false
}

fn parse_input(input: String) -> (i32, i32, i32, i32) {
    let schedules: Vec<&str> = input.split(",").collect();
    let first = schedules.get(0).expect("No value");
    let second = schedules.get(1).expect("No value");
    let firsts: Vec<i32> = first
        .split("-")
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let first_start = firsts.get(0).expect("No value");
    let first_end = firsts.get(1).expect("No value");
    let seconds: Vec<i32> = second
        .split("-")
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let second_start = seconds.get(0).expect("No value");
    let second_end = seconds.get(1).expect("No value");
    (*first_start, *first_end, *second_start, *second_end)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_is_fully_contain() {
        assert_eq!(is_fully_contain("2-4,6-8".to_string()), false);
        assert_eq!(is_fully_contain("2-3,4-5".to_string()), false);
        assert_eq!(is_fully_contain("5-7,7-9".to_string()), false);
        assert_eq!(is_fully_contain("2-8,3-7".to_string()), true);
        assert_eq!(is_fully_contain("6-6,4-6".to_string()), true);
        assert_eq!(is_fully_contain("2-6,4-8".to_string()), false);
    }

    #[test]
    fn test_is_partly_contain() {
        assert_eq!(is_partly_contain("2-4,6-8".to_string()), false);
        assert_eq!(is_partly_contain("2-3,4-5".to_string()), false);
        assert_eq!(is_partly_contain("5-7,7-9".to_string()), true);
        assert_eq!(is_partly_contain("2-8,3-7".to_string()), true);
        assert_eq!(is_partly_contain("6-6,4-6".to_string()), true);
        assert_eq!(is_partly_contain("2-6,4-8".to_string()), true);
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day4_part1(PathBuf::from("src/solution/s04/example.txt")),
            2
        );
        assert_eq!(
            solution_day4_part1(PathBuf::from("src/solution/s04/input.txt")),
            526
        );
        assert_eq!(
            solution_day4_part2(PathBuf::from("src/solution/s04/example.txt")),
            4
        );
        assert_eq!(
            solution_day4_part2(PathBuf::from("src/solution/s04/input.txt")),
            886
        );
    }
}
