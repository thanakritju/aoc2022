use std::collections::{BinaryHeap, HashSet};

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day8_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let width = input.get(0).expect("No data").len();
    let height = input.len();
    let mut grid_raw = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid = grid_base.as_mut_slice();
    for (j, line) in input.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            grid[j][i] = c.to_digit(10).unwrap()
        }
    }

    let mut visited: HashSet<(usize, usize)> = Default::default();
    let mut visible: HashSet<(usize, usize)> = Default::default();
    let mut queue: Vec<(usize, usize)> = Default::default();
    queue.push((0, 0));
    while !queue.is_empty() {
        let (i, j) = queue.pop().expect("no data");
        visited.insert((i, j));

        if is_in_the_edge(width, height, i, j) {
            visible.insert((i, j));
        } else {
            if visible.get(&(i, j)).is_none() {
                if is_visible(width, height, i, j, grid) {
                    visible.insert((i, j));
                }
            }
        }

        for (ni, nj) in get_neighbors(width, height, i, j) {
            let is_visited = visited.get(&(ni, nj)).is_some();
            if !is_visited {
                queue.push((ni, nj))
            }
        }
    }
    visible.len().try_into().unwrap()
}

pub fn solution_day8_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let width = input.get(0).expect("No data").len();
    let height = input.len();
    let mut grid_raw = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid = grid_base.as_mut_slice();
    for (j, line) in input.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            grid[j][i] = c.to_digit(10).unwrap()
        }
    }

    let mut visited: HashSet<(usize, usize)> = Default::default();
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut queue: Vec<(usize, usize)> = Default::default();
    queue.push((0, 0));
    while !queue.is_empty() {
        let (i, j) = queue.pop().expect("no data");
        visited.insert((i, j));

        if !is_in_the_edge(width, height, i, j) {
            heap.push(get_score(width, height, i, j, grid));
        }

        for (ni, nj) in get_neighbors(width, height, i, j) {
            let is_visited = visited.get(&(ni, nj)).is_some();
            if !is_visited {
                queue.push((ni, nj))
            }
        }
    }
    *heap.peek().expect("No data")
}

pub fn is_visible(
    width: usize,
    height: usize,
    i: usize,
    j: usize,
    grid: &mut [&mut [u32]],
) -> bool {
    let h = grid[j][i];
    get_bottoms(width, height, i, j)
        .into_iter()
        .all(|(x, y)| h > grid[y][x])
        || get_rights(width, height, i, j)
            .into_iter()
            .all(|(x, y)| h > grid[y][x])
        || get_lefts(width, height, i, j)
            .into_iter()
            .all(|(x, y)| h > grid[y][x])
        || get_tops(width, height, i, j)
            .into_iter()
            .all(|(x, y)| h > grid[y][x])
}

pub fn get_score(width: usize, height: usize, i: usize, j: usize, grid: &mut [&mut [u32]]) -> i32 {
    let mut score = 1;
    let h = grid[j][i];
    let mut count = 0;
    for (x, y) in get_bottoms(width, height, i, j) {
        if h > grid[y][x] {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    score *= count;
    count = 0;
    for (x, y) in get_lefts(width, height, i, j) {
        if h > grid[y][x] {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    score *= count;
    count = 0;
    for (x, y) in get_tops(width, height, i, j) {
        if h > grid[y][x] {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    score *= count;
    count = 0;
    for (x, y) in get_rights(width, height, i, j) {
        if h > grid[y][x] {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    score *= count;
    println!("x {} y {} h {} score {}", i, j, h, score);
    score
}

pub fn is_in_the_edge(width: usize, height: usize, i: usize, j: usize) -> bool {
    if i == 0 || i == width - 1 {
        return true;
    }
    if j == 0 || j == height - 1 {
        return true;
    }
    return false;
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

pub fn get_rights(width: usize, height: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    let mut x = i + 1;
    while x < width {
        vec.push((x, j));
        x += 1;
    }
    vec
}

pub fn get_lefts(width: usize, height: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    let mut x = i - 1;
    while x > 0 {
        vec.push((x, j));
        x -= 1;
    }
    vec.push((x, j));
    vec
}

pub fn get_tops(width: usize, height: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    let mut y = j + 1;
    while y < height {
        vec.push((i, y));
        y += 1;
    }
    vec
}

pub fn get_bottoms(width: usize, height: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    let mut y = j - 1;
    while y > 0 {
        vec.push((i, y));
        y -= 1;
    }
    vec.push((i, y));
    vec
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_get_directions() {
        assert_eq!(get_lefts(5, 5, 1, 3), vec![(0, 3)]);
        assert_eq!(get_rights(5, 5, 1, 3), vec![(2, 3), (3, 3), (4, 3)]);
        assert_eq!(get_tops(5, 5, 1, 3), vec![(1, 4)]);
        assert_eq!(get_bottoms(5, 5, 1, 3), vec![(1, 2), (1, 1), (1, 0)]);
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day8_part1(PathBuf::from("src/solution/s08/example.txt")),
            21
        );
        assert_eq!(
            solution_day8_part1(PathBuf::from("src/solution/s08/input.txt")),
            1708
        );
        assert_eq!(
            solution_day8_part2(PathBuf::from("src/solution/s08/example.txt")),
            8
        );
        assert_eq!(
            solution_day8_part2(PathBuf::from("src/solution/s08/input.txt")),
            504000
        );
    }
}
