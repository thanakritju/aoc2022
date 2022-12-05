
pub fn solution_day20_part1(path: std::path::PathBuf) -> i32 {
    0
}

pub fn solution_day20_part2(path: std::path::PathBuf) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day20_part1(PathBuf::from("src/solution/20/example.txt")),
            0
        );
        assert_eq!(
            solution_day20_part1(PathBuf::from("src/solution/s20/input.txt")),
            0
        );
        assert_eq!(
            solution_day20_part2(PathBuf::from("src/solution/s20/example.txt")),
            0
        );
        assert_eq!(
            solution_day20_part2(PathBuf::from("src/solution/s20/input.txt")),
            0
        );
    }
}
