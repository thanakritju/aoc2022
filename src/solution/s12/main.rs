use std::{collections::HashSet, usize::MAX};

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day12_part1(path: std::path::PathBuf) -> usize {
    let input = load_file_to_string_vectors(path);
    let width = input.get(0).expect("No data").len();
    let height = input.len();
    let mut grid_raw = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid = grid_base.as_mut_slice();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (j, line) in input.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == 'E' {
                end = (i, j);
            }
            if c == 'S' {
                start = (i, j);
            }
            grid[j][i] = char_to_num(c);
        }
    }
    search(start, end, width, height, grid)
}

fn search(
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
    grid: &mut [&mut [usize]],
) -> usize {
    let mut visited: HashSet<(usize, usize)> = Default::default();
    let mut queue: Vec<(usize, usize, usize)> = Default::default();
    let (x, y) = start;
    let mut ans = MAX;
    queue.push((x, y, 0));
    visited.insert((x, y));
    while !queue.is_empty() {
        let (i, j, d) = queue.pop().expect("no data");
        if (i, j) == end {
            if d < ans {
                ans = d;
            }
        }
        let h = grid[j][i];

        for (ni, nj) in get_neighbors(width, height, i, j) {
            let nh = grid[nj][ni];
            let is_visited = visited.get(&(ni, nj)).is_some();
            if !is_visited && nh <= 1 + h {
                queue.push((ni, nj, d + 1));
                visited.insert((ni, nj));
            }
        }
        queue.sort_by(|a, b| b.2.cmp(&a.2));
    }
    ans
}

pub fn get_neighbors(width: usize, height: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    if i > 0 {
        vec.push((i - 1, j))
    }
    if j > 0 {
        vec.push((i, j - 1))
    }
    if i < width - 1 {
        vec.push((i + 1, j))
    }
    if j < height - 1 {
        vec.push((i, j + 1))
    }
    vec
}

pub fn solution_day12_part2(path: std::path::PathBuf) -> usize {
    let input = load_file_to_string_vectors(path);
    let width = input.get(0).expect("No data").len();
    let height = input.len();
    let mut grid_raw = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid = grid_base.as_mut_slice();
    let mut starts = vec![];
    let mut end = (0, 0);
    for (j, line) in input.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == 'E' {
                end = (i, j);
            }
            if c == 'S' || c == 'a' {
                starts.push((i, j));
            }
            grid[j][i] = char_to_num(c);
        }
    }
    let mut min = MAX;
    for each in starts {
        let out = search(each, end, width, height, grid);
        if out < min {
            min = out
        }
    }
    min
}

fn char_to_num(c: char) -> usize {
    let alphabets = "abcdefghijklmnopqrstuvwxyz";
    match c {
        'S' => 0,
        'E' => 25,
        c => match alphabets.find(c) {
            Some(v) => (v).try_into().unwrap(),
            None => panic!("unregonized char"),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day12_part1(PathBuf::from("src/solution/s12/example.txt")),
            31
        );
        assert_eq!(
            solution_day12_part1(PathBuf::from("src/solution/s12/input.txt")),
            394
        );
        assert_eq!(
            solution_day12_part2(PathBuf::from("src/solution/s12/example.txt")),
            29
        );
        assert_eq!(
            solution_day12_part2(PathBuf::from("src/solution/s12/input.txt")),
            388
        );
    }
}
