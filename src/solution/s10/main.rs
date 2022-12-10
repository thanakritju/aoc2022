use std::collections::VecDeque;

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day10_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    vec![20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| signal_strength(process2(cycle, input.to_owned()), cycle as i32))
        .sum()
}

pub fn solution_day10_part2(path: std::path::PathBuf) -> i32 {
    0
}

fn signal_strength(register: i32, cycle: i32) -> i32 {
    register * cycle
}

fn process(target_cycle: usize, program: Vec<String>) -> i32 {
    let mut register = 1;
    let len = program.len();
    let mut i: usize = 0;
    let mut action_index: usize = 0;
    while i <= target_cycle {
        let action = program.get(action_index % len).expect("no data");
        println!(
            "Start : cycle {} register {} action {}",
            i + 1,
            register,
            action
        );
        if action.starts_with("addx ") {
            let splited: Vec<&str> = action.split("addx ").collect();
            let num: i32 = splited
                .get(1)
                .expect("no data")
                .parse()
                .expect("cannot parse");
            if i < target_cycle - 1 {
                register += num;
            }
            action_index += 1;
            i += 2;
        } else {
            action_index += 1;
            i += 1;
        }
    }
    // println!("----");
    register
}

fn process2(target_cycle: usize, program: Vec<String>) -> i32 {
    let mut register = 1;
    let mut mem = 0;
    let len = program.len();
    let mut i: usize = 0;
    let mut action_index: usize = 0;
    let mut pause = false;
    let mut q: VecDeque<i32> = VecDeque::from([0]);
    while i <= target_cycle {
        println!("Start : cycle {} register {}", i + 1, register);

        let num = q.pop_front().expect("data");
        register += num;
        let action = program.get(action_index % len).expect("no data");
        if action.starts_with("addx ") {
            let splited: Vec<&str> = action.split("addx ").collect();
            let num: i32 = splited
                .get(1)
                .expect("no data")
                .parse()
                .expect("cannot parse");
            if q.is_empty() {
                q.push_back(num);
                action_index += 1;
            }
        } else {
            action_index += 1;
        }
        q.push_back(0);
        println!("Finish: cycle {} register {}", i + 1, register);
        i += 1;
    }
    if !pause {
        register += mem;
    }
    println!("----");
    register
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_process() {
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process(1, program), 1);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process(2, program), 1);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process(3, program), 4);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process(4, program), 4);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process(5, program), -1);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process(10, program), -3);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process(20, program), 21);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process(60, program), 19);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process(100, program), 18);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process(140, program), 21);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process(180, program), 16);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process(220, program), 18);
    }

    // #[test]
    fn test_process2() {
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process2(1, program), 1);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process2(2, program), 1);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process2(3, program), 4);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process2(4, program), 4);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process2(5, program), -1);
        let program = load_file_to_string_vectors("src/solution/s10/small_example.txt");
        assert_eq!(process2(10, program), -3);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process2(20, program), 21);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process2(60, program), 19);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process2(100, program), 18);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process2(140, program), 21);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process2(180, program), 16);
        let program = load_file_to_string_vectors("src/solution/s10/example.txt");
        assert_eq!(process2(220, program), 18);
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
