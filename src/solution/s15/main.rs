use std::{collections::HashSet, str::FromStr};

use regex::Regex;

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day15_part1(path: std::path::PathBuf) -> usize {
    let input = load_file_to_string_vectors(path);
    get_number(input, 2000000)
}

fn get_number(input: Vec<String>, row: i32) -> usize {
    // let mut set: HashSet<(i32, i32)> = Default::default();
    // for line in input {
    //     let sab = line.parse::<SensorAndBeacon>().unwrap();
    //     set.extend(get_those_in_teritory(sab, row))
    // }
    // set.len()
    let mut count = 0;
    for line in input {
        let sab = line.parse::<SensorAndBeacon>().unwrap();
        count += how_many_in_the_teritory(sab, row)
    }
    count
}

pub fn solution_day15_part2(path: std::path::PathBuf) -> i32 {
    0
}

fn how_many_in_the_teritory(sab: SensorAndBeacon, row: i32) -> usize {
    let mut count = 0;
    let d = sab.distance();
    let dy = (sab.1 - row).abs();
    let dx = d - dy;
    if dx <= 0 {
        return 0;
    }
    let dx_start = sab.0 - dx;
    let dx_end = sab.0 + dx;
    count += 2 * dx + 1;
    if row == sab.3 && (sab.2 == dx_start || sab.2 == dx_end) {
        count -= 1;
    }
    count.try_into().unwrap()
}

fn get_those_in_teritory(sab: SensorAndBeacon, row: i32) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = Default::default();
    let d = sab.distance();
    let dy = (sab.1 - row).abs();
    let dx = d - dy;
    if dx <= 0 {
        return set;
    }
    let dx_start = sab.0 - dx;
    let dx_end = sab.0 + dx;
    for x in dx_start..dx_end + 1 {
        if x == sab.2 && row == sab.3 {
        } else {
            set.insert((x, row));
        }
    }
    set
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
    fn test_get_those_in_teritory() {
        assert_eq!(
            get_those_in_teritory(SensorAndBeacon(8, 7, 2, 10), 10),
            HashSet::from([
                (3, 10),
                (4, 10),
                (5, 10),
                (6, 10),
                (7, 10),
                (8, 10),
                (9, 10),
                (10, 10),
                (11, 10),
                (12, 10),
                (13, 10),
                (14, 10)
            ])
        );
    }

    #[test]
    fn test_how_many_in_the_teritory() {
        assert_eq!(
            how_many_in_the_teritory(SensorAndBeacon(8, 7, 2, 10), 10),
            12
        );
        assert_eq!(
            how_many_in_the_teritory(SensorAndBeacon(8, 7, 14, 10), 10),
            12
        );
        assert_eq!(
            how_many_in_the_teritory(SensorAndBeacon(8, 7, 2, 10), 17),
            0
        );
        assert_eq!(
            how_many_in_the_teritory(SensorAndBeacon(0, 11, 2, 10), 10),
            4
        );
        assert_eq!(
            how_many_in_the_teritory(SensorAndBeacon(8, 7, 2, 10), 9),
            15
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
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day15_part1(PathBuf::from("src/solution/s15/input.txt")),
            0
        );
        // assert_eq!(
        //     solution_day15_part2(PathBuf::from("src/solution/s15/example.txt")),
        //     0
        // );
        // assert_eq!(
        //     solution_day15_part2(PathBuf::from("src/solution/s15/input.txt")),
        //     0
        // );
    }
}
