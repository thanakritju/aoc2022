
pub fn solution_day24_part1(path: std::path::PathBuf) -> i32 {
    0
}

pub fn solution_day24_part2(path: std::path::PathBuf) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day24_part1(PathBuf::from("src/solution/24/example.txt")),
            0
        );
        assert_eq!(
            solution_day24_part1(PathBuf::from("src/solution/s24/input.txt")),
            0
        );
        assert_eq!(
            solution_day24_part2(PathBuf::from("src/solution/s24/example.txt")),
            0
        );
        assert_eq!(
            solution_day24_part2(PathBuf::from("src/solution/s24/input.txt")),
            0
        );
    }
}
