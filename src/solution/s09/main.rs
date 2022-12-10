use std::collections::HashSet;

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day9_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let actions = parse_input(input);
    let (mut sx, mut sy): (i32, i32) = (0, 0);
    let mut t: HashSet<(i32, i32)> = HashSet::new();
    let (mut tx, mut ty): (i32, i32) = (0, 0);
    for (action, number) in actions {
        for _ in 0..number {
            let (x, y) = get_new_head(sx, sy, action.to_owned());
            (tx, ty) = update_tail_v2(x, y, tx, ty);
            (sx, sy) = (x, y);
            t.insert((tx, ty));
        }
    }
    t.len().try_into().unwrap()
}

pub fn solution_day9_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let actions = parse_input(input);
    let mut array: [Vec<(i32, i32)>; 10] = Default::default();
    for i in 0..10 {
        array[i].push((0, 0));
    }
    for (action, number) in actions {
        array = update_array_v2(array, action.to_owned(), number);
    }
    let set: HashSet<_> = array[9].iter().cloned().collect();
    set.len().try_into().unwrap()
}

fn update_array_v2(
    mut array: [Vec<(i32, i32)>; 10],
    action: String,
    times: usize,
) -> [Vec<(i32, i32)>; 10] {
    for _ in 0..times {
        let (x, y) = array[0].last().expect("No data");
        array[0].push(get_new_head(*x, *y, action.to_owned()));
        for i in 1..10 {
            let (sx, sy) = array[i - 1].last().expect("No data");
            let (tx, ty) = array[i].last().expect("No data");
            array[i].push(update_tail_v2(*sx, *sy, *tx, *ty));
        }
    }
    array
}

fn get_new_head(sx: i32, sy: i32, action: String) -> (i32, i32) {
    let (mut dx, mut dy) = (0, 0);
    match action.as_str() {
        "U" => {
            dy = 1;
        }
        "D" => {
            dy = -1;
        }
        "L" => {
            dx = -1;
        }
        "R" => {
            dx = 1;
        }
        _ => panic!("action isn't regonized"),
    }
    (sx + dx, sy + dy)
}

fn update_tail_v2(sx: i32, sy: i32, tx: i32, ty: i32) -> (i32, i32) {
    let dist = (sx - tx).abs() + (sy - ty).abs();
    if sx == tx && dist >= 2 {
        if sy > ty {
            return (tx, sy - 1);
        } else {
            return (tx, sy + 1);
        }
    }
    if sy == ty && dist >= 2 {
        if sx > tx {
            return (sx - 1, ty);
        } else {
            return (sx + 1, ty);
        }
    }
    if dist > 2 {
        if sx > tx {
            if sy > ty {
                return (tx + 1, ty + 1);
            } else {
                return (tx + 1, ty - 1);
            }
        }
        if sx < tx {
            if sy > ty {
                return (tx - 1, ty + 1);
            } else {
                return (tx - 1, ty - 1);
            }
        }
    }
    return (tx, ty);
}

fn parse_input(lines: Vec<String>) -> Vec<(String, usize)> {
    let mut vec: Vec<(String, usize)> = vec![];
    for line in lines {
        let mut splited = line.split(" ");
        let action = splited.next().expect("no data").to_string();
        let number: usize = splited
            .next()
            .expect("no data")
            .parse()
            .expect("parse error");
        vec.push((action.to_string(), number))
    }
    vec
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_update_tail_v2() {
        assert_eq!(update_tail_v2(4, 2, 3, 0), (4, 1));
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day9_part1(PathBuf::from("src/solution/s09/example.txt")),
            13
        );
        assert_eq!(
            solution_day9_part1(PathBuf::from("src/solution/s09/input.txt")),
            6745
        );
        assert_eq!(
            solution_day9_part2(PathBuf::from("src/solution/s09/example.txt")),
            1
        );
        assert_eq!(
            solution_day9_part2(PathBuf::from("src/solution/s09/example2.txt")),
            36
        );
        assert_eq!(
            solution_day9_part2(PathBuf::from("src/solution/s09/input.txt")),
            2793
        );
    }
}
