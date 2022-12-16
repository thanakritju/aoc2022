use std::str::FromStr;

use regex::Regex;

pub fn solution_day16_part1(path: std::path::PathBuf) -> i32 {
    0
}

pub fn solution_day16_part2(path: std::path::PathBuf) -> i32 {
    0
}

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
struct Valve {
    pub name: String,
    pub rate: i32,
    pub is_open: bool,
    pub conected_values: Vec<String>,
}

impl Valve {
    pub fn score(self, distance_to_valve: i32, time_remaining: i32) -> i32 {
        0
    }
}

impl FromStr for Valve {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Valve (\w\w) has flow rate=([\d]+); tunnels? leads? to valves? ([\w\\,\\ ]*)",
        );
        let caps = re?.captures(s).unwrap();
        let name = caps.get(1).map_or("0", |m| m.as_str());
        let num = caps.get(2).map_or("0", |m| m.as_str());
        let list = caps.get(3).map_or("0", |m| m.as_str());

        let rate = num.parse::<i32>().unwrap();
        let name = String::from(name);
        let conected_values: Vec<String> = list
            .to_string()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        Ok(Valve {
            name,
            rate,
            is_open: false,
            conected_values,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse() {
        let valve = "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE"
            .parse::<Valve>()
            .unwrap();
        assert_eq!(
            valve,
            Valve {
                name: "DD".to_string(),
                rate: 20,
                is_open: false,
                conected_values: vec!["CC".to_string(), "AA".to_string(), "EE".to_string()]
            }
        );
        let valve = "
        Valve HH has flow rate=22; tunnel leads to valve GG"
            .parse::<Valve>()
            .unwrap();
        assert_eq!(
            valve,
            Valve {
                name: "HH".to_string(),
                rate: 22,
                is_open: false,
                conected_values: vec!["GG".to_string()]
            }
        );
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day16_part1(PathBuf::from("src/solution/s16/example.txt")),
            1651
        );
        assert_eq!(
            solution_day16_part1(PathBuf::from("src/solution/s16/input.txt")),
            0
        );
        assert_eq!(
            solution_day16_part2(PathBuf::from("src/solution/s16/example.txt")),
            0
        );
        assert_eq!(
            solution_day16_part2(PathBuf::from("src/solution/s16/input.txt")),
            0
        );
    }
}
