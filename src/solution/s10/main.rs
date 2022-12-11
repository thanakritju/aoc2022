use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day10_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let registers: [i32; 220] = process(input);
    vec![20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| signal_strength(registers[cycle - 1], cycle as i32))
        .sum()
}

pub fn solution_day10_part2(path: std::path::PathBuf) -> i32 {
    0
}

fn signal_strength(register: i32, cycle: i32) -> i32 {
    register * cycle
}

fn process<const LEN: usize>(actions: Vec<String>) -> [i32; LEN] {
    let mut arr = [1; LEN];
    let mut ai = 0;
    let mut pause = false;
    let mut num = 0;
    for i in 0..LEN {
        if i > 0 {
            arr[i] = arr[i - 1];
        }
        if !pause {
            let action = actions.get(ai % actions.len()).expect("no data");
            if action.starts_with("addx") {
                let splited: Vec<&str> = action.split("addx ").collect();
                num = splited
                    .get(1)
                    .expect("no data")
                    .parse()
                    .expect("parse error");
                arr[i] += num;
                pause = true;
                ai += 1;
            } else {
                pause = false;
                ai += 1;
            }
        } else {
            pause = true;
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_process() {
        let actions = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        let registers: [i32; 5] = process(actions);
        assert_eq!(registers[0], 1);
        assert_eq!(registers[1], 1);
        assert_eq!(registers[2], 4);
        assert_eq!(registers[3], 4);
        assert_eq!(registers[4], -1);
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day10_part1(PathBuf::from("src/solution/s10/example.txt")),
            13140
        );
        assert_eq!(
            solution_day10_part1(PathBuf::from("src/solution/s10/input.txt")),
            0
        );
        assert_eq!(
            solution_day10_part2(PathBuf::from("src/solution/s10/example.txt")),
            0
        );
        assert_eq!(
            solution_day10_part2(PathBuf::from("src/solution/s10/input.txt")),
            0
        );
    }
}
