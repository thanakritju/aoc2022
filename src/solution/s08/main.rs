use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day8_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let grid = parse_input(input);
    0
}

pub fn solution_day8_part2(path: std::path::PathBuf) -> i32 {
    0
}

fn parse_input(input: Vec<String>) -> Grid {
    Grid::new(input)
}

#[derive(Copy, Clone)]
struct Tree {
    x: i32,
    y: i32,
    h: i32,
}

impl Tree {
    pub fn new(x: i32, y: i32, h: i32) -> Tree {
        return Tree { x: x, y: y, h: h };
    }
}

struct Grid {
    trees: Vec<Vec<Tree>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn new(lines: Vec<String>) -> Grid {
        let mut grid = Grid {
            trees: Default::default(),
            max_x: lines.get(0).expect("No data").len(),
            max_y: lines.len(),
        };
        for (i, line) in lines.iter().enumerate() {
            grid.add_line(line, i as i32)
        }

        return grid;
    }

    fn add_line(&mut self, line: &String, nrow: i32) {
        let mut row: Vec<Tree> = Default::default();
        for (i, c) in line.chars().enumerate() {
            let tree = Tree::new(i as i32, nrow as i32, c as i32);
            row.push(tree)
        }
    }

    pub fn get_tree_height(self, i: usize, j: usize) -> i32 {
        let row = self.trees.get(i).expect("No data");
        row.get(j).expect("No data").h
    }

    pub fn get_tree(self, i: usize, j: usize) -> Tree {
        let row = self.trees.get(i).expect("No data");
        row.get(j).expect("No data").clone()
    }

    pub fn is_tree_visible(self, i: usize, j: usize) -> bool {
        if self.is_in_the_edge(i, j) {
            return true;
        }
        return false;
    }

    pub fn get_neibours(self, i: usize, j: usize) -> Vec<Tree> {
        let mut neibours = vec![];
        for (ni, nj) in vec![(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if self.is_valid_grid(ni, nj) {
                neibours.push(self.get_tree(ni, nj))
            }
        }
        neibours
    }

    pub fn is_in_the_edge(self, i: usize, j: usize) -> bool {
        if i == 0 || i == self.max_x - 1 {
            return true;
        }
        if j == 0 || j == self.max_y - 1 {
            return true;
        }
        return false;
    }

    pub fn is_valid_grid(self, i: usize, j: usize) -> bool {
        if i < 0 || i > self.max_x - 1 {
            return false;
        }
        if j < 0 || j > self.max_y - 1 {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_is_tree_visible() {
        let input = load_file_to_string_vectors("src/solution/s08/example.txt");
        assert_eq!(parse_input(input.to_owned()).is_tree_visible(0, 0), true);
        assert_eq!(parse_input(input.to_owned()).is_tree_visible(3, 1), false);
        assert_eq!(parse_input(input.to_owned()).is_tree_visible(3, 2), true);
        assert_eq!(parse_input(input.to_owned()).is_tree_visible(3, 3), false);
        assert_eq!(parse_input(input.to_owned()).is_tree_visible(4, 1), true);
    }

    #[test]
    fn test_is_in_the_edge() {
        let input = load_file_to_string_vectors("src/solution/s08/example.txt");
        assert_eq!(parse_input(input.to_owned()).is_in_the_edge(0, 0), true);
        assert_eq!(parse_input(input.to_owned()).is_in_the_edge(1, 1), false);
        assert_eq!(parse_input(input.to_owned()).is_in_the_edge(1, 4), true);
        assert_eq!(parse_input(input.to_owned()).is_in_the_edge(4, 1), true);
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day8_part1(PathBuf::from("src/solution/s08/example.txt")),
            21
        );
        assert_eq!(
            solution_day8_part1(PathBuf::from("src/solution/s08/input.txt")),
            0
        );
        assert_eq!(
            solution_day8_part2(PathBuf::from("src/solution/s08/example.txt")),
            0
        );
        assert_eq!(
            solution_day8_part2(PathBuf::from("src/solution/s08/input.txt")),
            0
        );
    }
}
