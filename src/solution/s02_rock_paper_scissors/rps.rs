use std::collections::HashMap;

fn rps(rounds: Vec<String>) -> i32 {
    let mut score = 0;
    for round in rounds {
        score += get_score(round)
    }
    score
}

fn rps2(rounds: Vec<String>) -> i32 {
    let mut score = 0;
    for round in rounds {
        score += get_score2(round)
    }
    score
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

#[cfg(test)]
mod tests {
    use crate::util::load_file::load_file_to_string_vectors;

    use super::*;

    #[test]
    fn test_rps() {
        let input = load_file_to_string_vectors(String::from(
            "src/solution/s02_rock_paper_scissors/example.txt",
        ));
        assert_eq!(rps(input), 15);
        let input = load_file_to_string_vectors(String::from(
            "src/solution/s02_rock_paper_scissors/input.txt",
        ));
        assert_eq!(rps(input), 12772);
        let input = load_file_to_string_vectors(String::from(
            "src/solution/s02_rock_paper_scissors/example.txt",
        ));
        assert_eq!(rps2(input), 12);
        let input = load_file_to_string_vectors(String::from(
            "src/solution/s02_rock_paper_scissors/input.txt",
        ));
        assert_eq!(rps2(input), 12772);
    }
}
