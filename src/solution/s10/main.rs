use std::collections::VecDeque;

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day10_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let registers: [i32; 221] = process(input);
    vec![20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| signal_strength(registers[cycle - 1], cycle as i32))
        .sum()
}

pub fn solution_day10_part2(path: std::path::PathBuf) -> String {
    let input = load_file_to_string_vectors(path);
    let registers: [i32; 241] = process(input);
    println!("{:?}", registers);
    let mut ans = "".to_string();
    for i in 0..6 {
        let mut line = "\n".to_string();
        for j in 0..40 {
            let sprite_loc = registers[40 * i + j];
            if j as i32 <= sprite_loc + 1 && j as i32 >= sprite_loc - 1 {
                line = format!("{}{}", line, "#")
            } else {
                line = format!("{}{}", line, ".")
            }
        }
        ans = format!("{}{}", ans, line)
    }
    ans
}

fn signal_strength(register: i32, cycle: i32) -> i32 {
    register * cycle
}

fn process<const LEN: usize>(actions: Vec<String>) -> [i32; LEN] {
    let mut arr = [1; LEN];
    let mut ai = 0;
    let mut q: VecDeque<i32> = VecDeque::from([0, 0]);
    for i in 0..LEN {
        if i > 0 {
            arr[i] = arr[i - 1];
        }
        let n = q.pop_front().expect("No data");
        arr[i] = arr[i] + n;

        if q.iter().all(|e| *e == 0) {
            let action = actions.get(ai % actions.len()).expect("no data");
            if action.starts_with("addx") {
                let splited: Vec<&str> = action.split("addx ").collect();
                let num = splited
                    .get(1)
                    .expect("no data")
                    .parse()
                    .expect("parse error");
                q.push_back(num);
                ai += 1;
            } else {
                q.push_back(0);
                ai += 1;
            }
        } else {
            q.push_back(0);
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
        let registers: [i32; 6] = process(actions);
        assert_eq!(registers, [1, 1, 1, 4, 4, -1]);
        assert_eq!(registers[0], 1);
        assert_eq!(registers[1], 1);
        assert_eq!(registers[2], 1);
        assert_eq!(registers[3], 4);
        assert_eq!(registers[4], 4);
        assert_eq!(registers[5], -1);
        let actions = load_file_to_string_vectors("src/solution/s10/example.txt");
        let registers: [i32; 221] = process(actions);
        assert_eq!(registers[20], 21);
        assert_eq!(registers[60], 19);
        assert_eq!(registers[100], 18);
        assert_eq!(registers[140], 21);
        assert_eq!(registers[180], 16);
        assert_eq!(registers[219], 18);
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day10_part1(PathBuf::from("src/solution/s10/example.txt")),
            13140
        );
        assert_eq!(
            solution_day10_part1(PathBuf::from("src/solution/s10/input.txt")),
            14720
        );
        assert_eq!(
            solution_day10_part2(PathBuf::from("src/solution/s10/example.txt")),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string()
        );
        assert_eq!(
            solution_day10_part2(PathBuf::from("src/solution/s10/input.txt")),
            "
####.####.###..###..###..####.####.####.
#.......#.#..#.#..#.#..#.#.......#.#....
###....#..###..#..#.###..###....#..###..
#.....#...#..#.###..#..#.#.....#...#....
#....#....#..#.#....#..#.#....#....#....
#....####.###..#....###..#....####.#...."
                .to_string()
        );
    }
}
