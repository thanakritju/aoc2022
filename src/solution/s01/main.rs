use crate::utils::load_file::load_file_to_vectors;
use std::collections::BinaryHeap;

pub fn solution_day1_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_vectors(path);
    calorie_counting(input)
}
pub fn solution_day1_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_vectors(path);
    calorie_counting_top_three(input)
}

fn calorie_counting(numbers: Vec<Vec<i32>>) -> i32 {
    let mut sums: BinaryHeap<i32> = numbers.iter().map(|n| n.iter().sum()).collect();
    match sums.pop() {
        Some(v) => v,
        None => 0,
    }
}

fn calorie_counting_top_three(numbers: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut sums: BinaryHeap<i32> = numbers.iter().map(|n| n.iter().sum()).collect();
    match sums.pop() {
        Some(v) => ans += v,
        None => {}
    }
    match sums.pop() {
        Some(v) => ans += v,
        None => {}
    }
    match sums.pop() {
        Some(v) => ans += v,
        None => {}
    }
    ans
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day1_part1(PathBuf::from("src/solution/s01/example.txt")),
            24000
        );
        assert_eq!(
            solution_day1_part1(PathBuf::from("src/solution/s01/input.txt")),
            69626
        );
        assert_eq!(
            solution_day1_part2(PathBuf::from("src/solution/s01/example.txt")),
            45000
        );
        assert_eq!(
            solution_day1_part2(PathBuf::from("src/solution/s01/input.txt")),
            206780
        );
    }
}
