use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day3_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    input
        .into_iter()
        .map(find_unique_in_two)
        .map(char_to_int)
        .collect::<Vec<i32>>()
        .into_iter()
        .sum()
}

pub fn solution_day3_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    input
        .chunks(3)
        .map(|chunk| {
            find_unique_in_three(
                chunk.get(0).expect("no data"),
                chunk.get(1).expect("no data"),
                chunk.get(2).expect("no data"),
            )
        })
        .map(char_to_int)
        .collect::<Vec<i32>>()
        .into_iter()
        .sum()
}

fn find_unique_in_two(input: String) -> char {
    let (first, last) = input.split_at(input.len() / 2);
    for c1 in first.chars() {
        for c2 in last.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }
    '-'
}

fn find_unique_in_three(first: &String, mid: &String, last: &String) -> char {
    for c1 in first.chars() {
        for c2 in mid.chars() {
            for c3 in last.chars() {
                if c1 == c2 && c2 == c3 {
                    return c1;
                }
            }
        }
    }
    '-'
}

fn char_to_int(c: char) -> i32 {
    let alphabets = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    match alphabets.find(c) {
        Some(v) => (v + 1).try_into().unwrap(),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_char_to_int() {
        assert_eq!(char_to_int('p'), 16);
        assert_eq!(char_to_int('L'), 38);
        assert_eq!(char_to_int('P'), 42);
        assert_eq!(char_to_int('v'), 22);
        assert_eq!(char_to_int('t'), 20);
        assert_eq!(char_to_int('s'), 19);
    }

    #[test]
    fn test_find_unique_in_two() {
        assert_eq!(
            find_unique_in_two(String::from("vJrwpWtwJgWrhcsFMMfFFhFp")),
            'p'
        );
        assert_eq!(
            find_unique_in_two(String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")),
            'L'
        );
        assert_eq!(find_unique_in_two(String::from("PmmdzqPrVvPwwTWBwg")), 'P');
        assert_eq!(
            find_unique_in_two(String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")),
            'v'
        );
        assert_eq!(find_unique_in_two(String::from("ttgJtRGJQctTZtZT")), 't');
        assert_eq!(
            find_unique_in_two(String::from("CrZsJsPPZsGzwwsLwLmpwMDw")),
            's'
        );
    }

    #[test]
    fn test_reorganize() {
        assert_eq!(
            solution_day3_part1(PathBuf::from("src/solution/s03/example.txt")),
            157
        );
        assert_eq!(
            solution_day3_part1(PathBuf::from("src/solution/s03/input.txt")),
            7997
        );
        assert_eq!(
            solution_day3_part2(PathBuf::from("src/solution/s03/example.txt")),
            70
        );
        assert_eq!(
            solution_day3_part2(PathBuf::from("src/solution/s03/input.txt")),
            2545
        );
    }
}
