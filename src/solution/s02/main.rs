use std::collections::HashMap;

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day2_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    rock_paper_scissors(input, get_score)
}

pub fn solution_day2_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    rock_paper_scissors(input, get_score2)
}

fn rock_paper_scissors(rounds: Vec<String>, f: fn(String) -> i32) -> i32 {
    rounds
        .into_iter()
        .map(f)
        .collect::<Vec<i32>>()
        .into_iter()
        .sum()
}

fn get_score(key: String) -> i32 {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("A X".to_string(), 1 + 3);
    map.insert("A Y".to_string(), 2 + 6);
    map.insert("A Z".to_string(), 3 + 0);
    map.insert("B X".to_string(), 1 + 0);
    map.insert("B Y".to_string(), 2 + 3);
    map.insert("B Z".to_string(), 3 + 6);
    map.insert("C X".to_string(), 1 + 6);
    map.insert("C Y".to_string(), 2 + 0);
    map.insert("C Z".to_string(), 3 + 3);
    match map.get(&key) {
        Some(score) => *score,
        None => 0,
    }
}

fn get_score2(key: String) -> i32 {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("A X".to_string(), 0 + 3);
    map.insert("A Y".to_string(), 3 + 1);
    map.insert("A Z".to_string(), 6 + 2);
    map.insert("B X".to_string(), 0 + 1);
    map.insert("B Y".to_string(), 3 + 2);
    map.insert("B Z".to_string(), 6 + 3);
    map.insert("C X".to_string(), 0 + 2);
    map.insert("C Y".to_string(), 3 + 3);
    map.insert("C Z".to_string(), 6 + 1);
    match map.get(&key) {
        Some(score) => *score,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day2_part1(PathBuf::from("src/solution/s02/example.txt")),
            15
        );
        assert_eq!(
            solution_day2_part1(PathBuf::from("src/solution/s02/input.txt")),
            12772
        );
        assert_eq!(
            solution_day2_part2(PathBuf::from("src/solution/s02/example.txt")),
            12
        );
        assert_eq!(
            solution_day2_part2(PathBuf::from("src/solution/s02/input.txt")),
            11618
        );
    }
}
