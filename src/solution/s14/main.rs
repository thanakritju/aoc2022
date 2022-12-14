use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day14_part1(path: std::path::PathBuf) -> usize {
    let input = load_file_to_string_vectors(path);
    let coordinates = parse_input(input);
    let mut obstacles: HashSet<Coordinate> = Default::default();
    let mut bottom: usize = 0;
    for group in coordinates {
        for i in 0..group.len() - 1 {
            let c1 = *group.get(i).unwrap();
            let c2 = *group.get(i + 1).unwrap();
            let cs = c1.get_line(c2);
            for c in cs {
                if c.1 > bottom {
                    bottom = c.1
                }
                obstacles.insert(c);
            }
        }
    }
    let mut sand_count = 0;
    loop {
        let mut current = Coordinate(500, 0);
        let mut rest = false;
        let mut done = false;
        while !rest {
            if obstacles
                .get(&Coordinate(current.0, current.1 + 1))
                .is_some()
            {
                if obstacles
                    .get(&Coordinate(current.0 - 1, current.1 + 1))
                    .is_some()
                {
                    if obstacles
                        .get(&Coordinate(current.0 + 1, current.1 + 1))
                        .is_some()
                    {
                        rest = true;
                        obstacles.insert(Coordinate(current.0, current.1));
                        sand_count += 1;
                    } else {
                        current = Coordinate(current.0 + 1, current.1 + 1);
                    }
                } else {
                    current = Coordinate(current.0 - 1, current.1 + 1);
                }
            } else {
                current = Coordinate(current.0, current.1 + 1);
            }
            if current.1 > bottom {
                done = true;
                break;
            }
        }

        if done {
            break;
        }
    }
    sand_count
}

pub fn solution_day14_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let coordinates = parse_input(input);
    let mut obstacles: HashSet<Coordinate> = Default::default();
    let mut bottom: usize = 0;
    for group in coordinates {
        for i in 0..group.len() - 1 {
            let c1 = *group.get(i).unwrap();
            let c2 = *group.get(i + 1).unwrap();
            let cs = c1.get_line(c2);
            for c in cs {
                if c.1 > bottom {
                    bottom = c.1
                }
                obstacles.insert(c);
            }
        }
    }
    bottom += 2;
    let mut sand_count = 0;
    loop {
        let mut current = Coordinate(500, 0);
        let mut rest = false;
        let mut done = false;
        while !rest {
            if obstacles
                .get(&Coordinate(current.0, current.1 + 1))
                .is_some()
            {
                if obstacles
                    .get(&Coordinate(current.0 - 1, current.1 + 1))
                    .is_some()
                {
                    if obstacles
                        .get(&Coordinate(current.0 + 1, current.1 + 1))
                        .is_some()
                    {
                        rest = true;
                        obstacles.insert(Coordinate(current.0, current.1));
                        sand_count += 1;
                    } else {
                        current = Coordinate(current.0 + 1, current.1 + 1);
                    }
                } else {
                    current = Coordinate(current.0 - 1, current.1 + 1);
                }
            } else if current.1 + 1 == bottom {
                rest = true;
                obstacles.insert(Coordinate(current.0, current.1));
                sand_count += 1;
            } else {
                current = Coordinate(current.0, current.1 + 1);
            }

            if obstacles.get(&Coordinate(500, 0)).is_some() {
                done = true;
                break;
            }
        }

        if done {
            break;
        }
    }
    sand_count
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Coordinate>> {
    input
        .into_iter()
        .map(|line| {
            line.split(" -> ")
                .map(|s| Coordinate::from_str(s).unwrap())
                .collect()
        })
        .collect()
}

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy)]
struct Coordinate(usize, usize);

impl FromStr for Coordinate {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').unwrap();

        let x_fromstr = x.parse::<usize>()?;
        let y_fromstr = y.parse::<usize>()?;

        Ok(Coordinate(x_fromstr, y_fromstr))
    }
}

impl Coordinate {
    pub fn get_line(self, c2: Coordinate) -> Vec<Coordinate> {
        let mut vec = vec![];
        if self.0 == c2.0 {
            if self.1 > c2.1 {
                for i in c2.1..self.1 + 1 {
                    vec.push(Coordinate(self.0, i))
                }
            } else {
                for i in self.1..c2.1 + 1 {
                    vec.push(Coordinate(self.0, i))
                }
            }
        } else if self.1 == c2.1 {
            if self.0 > c2.0 {
                for i in c2.0..self.0 + 1 {
                    vec.push(Coordinate(i, self.1))
                }
            } else {
                for i in self.0..c2.0 + 1 {
                    vec.push(Coordinate(i, self.1))
                }
            }
        } else {
            panic!("cannot generate straight line")
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse() {
        let c1: Coordinate = "498,6".parse().expect("can't parse");
        let c2: Coordinate = "501,6".parse().expect("can't parse");
        assert_eq!(
            c1.get_line(c2),
            vec![
                Coordinate(498, 6),
                Coordinate(499, 6),
                Coordinate(500, 6),
                Coordinate(501, 6)
            ]
        );
        assert_eq!(
            Coordinate(503, 4).get_line(Coordinate(503, 1)),
            vec![
                Coordinate(503, 1),
                Coordinate(503, 2),
                Coordinate(503, 3),
                Coordinate(503, 4)
            ]
        );
    }

    #[test]
    fn test_get_line() {
        let out: Coordinate = "498,6".parse().expect("can't parse");
        assert_eq!(Coordinate(498, 6), out);
        assert_eq!(Coordinate(503, 4), Coordinate::from_str("503,4").unwrap());
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day14_part1(PathBuf::from("src/solution/s14/example.txt")),
            24
        );
        assert_eq!(
            solution_day14_part1(PathBuf::from("src/solution/s14/input.txt")),
            799
        );
        assert_eq!(
            solution_day14_part2(PathBuf::from("src/solution/s14/example.txt")),
            93
        );
    }

    #[ignore]
    #[test]
    fn test_slow_solution() {
        assert_eq!(
            solution_day14_part2(PathBuf::from("src/solution/s14/input.txt")),
            29076
        );
    }
}
