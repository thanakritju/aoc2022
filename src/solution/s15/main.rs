use std::{collections::HashSet, str::FromStr};

use regex::Regex;

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day15_part1(path: std::path::PathBuf) -> usize {
    let input = load_file_to_string_vectors(path);
    get_number(input, 2000000)
}

pub fn solution_day15_part2(path: std::path::PathBuf) -> usize {
    let input = load_file_to_string_vectors(path);
    let sabs: Vec<SensorAndBeacon> = input
        .into_iter()
        .map(|s| s.parse().expect("can't parse"))
        .collect();
    let mut beacons: HashSet<(i32, i32)> = Default::default();
    for sab in &sabs {
        beacons.insert((sab.2, sab.3));
    }
    let mut found = false;
    let mut x = 0;
    let mut y = 0;
    for row in 1..4000000 {
        let mut vec = vec![];
        for sab in &sabs {
            match get_lower_bound_and_upper_bound(*sab, row) {
                Some(x) => vec.push(x),
                None => {}
            }
        }
        if vec.is_empty() {
            continue;
        }
        vec.sort_by(|a, b| b.0.cmp(&a.0));
        while !vec.is_empty() {
            let last = vec.pop().unwrap();
            let second_last = vec.pop().unwrap();
            match merge(last, second_last) {
                Some(p) => {
                    if vec.is_empty() {
                    } else {
                        vec.push(p);
                    }
                }
                None => {
                    if last.1 + 2 == second_last.0 && beacons.get(&(last.1 + 1, row)).is_none() {
                        found = true;
                        x = last.1 + 1;
                        y = row;
                        break;
                    }
                    if vec.is_empty() {
                    } else {
                        vec.push(second_last);
                    }
                }
            }
        }
        if found {
            break;
        }
    }
    let usize_x: usize = x.try_into().unwrap();
    let usize_y: usize = y.try_into().unwrap();
    4000000 * usize_x + usize_y
}

fn get_number(input: Vec<String>, row: i32) -> usize {
    let mut vec = vec![];
    for line in input {
        let sab = line.parse::<SensorAndBeacon>().unwrap();
        match get_lower_bound_and_upper_bound(sab, row) {
            Some(x) => vec.push(x),
            None => {}
        }
    }
    let mut count = 0;
    vec.sort_by(|a, b| b.0.cmp(&a.0));
    while !vec.is_empty() {
        let last = vec.pop().unwrap();
        let second_last = vec.pop().unwrap();
        match merge(last, second_last) {
            Some(p) => {
                if vec.is_empty() {
                    count += p.1 - p.0 + 1;
                } else {
                    vec.push(p);
                }
            }
            None => {
                count += last.1 - last.0 + 1;
                if vec.is_empty() {
                    count += second_last.1 - second_last.0 + 1;
                } else {
                    vec.push(second_last)
                }
            }
        }
    }
    count.try_into().unwrap()
}

fn merge(p1: (i32, i32), p2: (i32, i32)) -> Option<(i32, i32)> {
    if p1.0 <= p2.0 && p1.1 >= p2.0 {
        if p1.1 >= p2.1 {
            return Some(p1);
        } else {
            return Some((p1.0, p2.1));
        }
    }
    if p2.0 <= p1.0 && p2.1 >= p1.0 {
        if p1.1 <= p2.1 {
            return Some(p2);
        } else {
            return Some((p2.0, p1.1));
        }
    }
    None
}

fn get_lower_bound_and_upper_bound(sab: SensorAndBeacon, row: i32) -> Option<(i32, i32)> {
    let d = sab.distance();
    let dy = (sab.1 - row).abs();
    let dx = d - dy;
    if dx <= 0 {
        return None;
    }
    let mut dx_start = sab.0 - dx;
    let mut dx_end = sab.0 + dx;
    if row == sab.3 {
        if sab.2 == dx_start {
            dx_start += 1
        }
        if sab.2 == dx_end {
            dx_end -= 1
        }
    }
    Some((dx_start, dx_end))
}

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy)]
struct SensorAndBeacon(i32, i32, i32, i32);

impl SensorAndBeacon {
    pub fn distance(self) -> i32 {
        (self.0 - self.2).abs() + (self.1 - self.3).abs()
    }
}

impl FromStr for SensorAndBeacon {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Sensor at x=([\d\\-]+), y=([\d\\-]+): closest beacon is at x=([\d\\-]+), y=([\d\\-]+)",
        );
        let caps = re?.captures(s).unwrap();
        let num1 = caps.get(1).map_or("0", |m| m.as_str());
        let num2 = caps.get(2).map_or("0", |m| m.as_str());
        let num3 = caps.get(3).map_or("0", |m| m.as_str());
        let num4 = caps.get(4).map_or("0", |m| m.as_str());

        let sx = num1.parse::<i32>().unwrap();
        let sy = num2.parse::<i32>().unwrap();
        let bx = num3.parse::<i32>().unwrap();
        let by = num4.parse::<i32>().unwrap();

        Ok(SensorAndBeacon(sx, sy, bx, by))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(SensorAndBeacon(8, 7, 2, 10).distance(), 9);
        assert_eq!(SensorAndBeacon(2, 18, -2, 15).distance(), 7);
    }

    #[test]
    fn test_merge() {
        assert_eq!(merge((1, 10), (1, 5)), Some((1, 10)));
        assert_eq!(merge((1, 10), (1, 15)), Some((1, 15)));
        assert_eq!(merge((1, 10), (-5, 15)), Some((-5, 15)));
        assert_eq!(merge((1, 10), (-5, 3)), Some((-5, 10)));
        assert_eq!(merge((1, 10), (8, 15)), Some((1, 15)));
        assert_eq!(merge((1, 10), (10, 15)), Some((1, 15)));
        assert_eq!(merge((1, 10), (11, 15)), None);
        assert_eq!(merge((16, 20), (11, 15)), None);
    }

    #[test]
    fn test_get_lower_bound_and_upper_bound() {
        assert_eq!(
            get_lower_bound_and_upper_bound(SensorAndBeacon(8, 7, 2, 10), 10),
            Some((3, 14))
        );
        assert_eq!(
            get_lower_bound_and_upper_bound(SensorAndBeacon(8, 7, 14, 10), 10),
            Some((2, 13))
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            "Sensor at x=60149, y=3320681: closest beacon is at x=-278961, y=3326224"
                .parse::<SensorAndBeacon>()
                .unwrap(),
            SensorAndBeacon(60149, 3320681, -278961, 3326224)
        )
    }

    #[test]
    fn test_get_number() {
        let input = load_file_to_string_vectors("src/solution/s15/example.txt");
        assert_eq!(get_number(input, 10), 26);
        let input = load_file_to_string_vectors("src/solution/s15/example.txt");
        assert_eq!(get_number(input, 3), 30);
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day15_part1(PathBuf::from("src/solution/s15/input.txt")),
            5144286
        );
        assert_eq!(
            solution_day15_part2(PathBuf::from("src/solution/s15/example.txt")),
            56000011
        );
    }

    #[ignore = "slow test"]
    #[test]
    fn test_slow_solution() {
        assert_eq!(
            solution_day15_part2(PathBuf::from("src/solution/s15/input.txt")),
            10229191267339
        );
    }
}
