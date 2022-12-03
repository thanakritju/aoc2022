use std::collections::HashMap;

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
    use crate::utils::load_file::load_file_to_string_vectors;

    use super::*;

    #[test]
    fn test_rps() {
        let input = load_file_to_string_vectors(String::from("src/solution/s02/example.txt"));
        assert_eq!(rock_paper_scissors(input, get_score), 15);
        let input = load_file_to_string_vectors(String::from("src/solution/s02/input.txt"));
        assert_eq!(rock_paper_scissors(input, get_score), 12772);
        let input = load_file_to_string_vectors(String::from("src/solution/s02/example.txt"));
        assert_eq!(rock_paper_scissors(input, get_score2), 12);
        let input = load_file_to_string_vectors(String::from("src/solution/s02/input.txt"));
        assert_eq!(rock_paper_scissors(input, get_score2), 11618);
    }
}
