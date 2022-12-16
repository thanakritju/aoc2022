use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use regex::Regex;

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day16_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let mut valves: HashMap<String, Valve> = Default::default();
    for line in input {
        let v = line.parse::<Valve>().unwrap();
        valves.insert(v.name.clone(), v);
    }
    0
}

pub fn solution_day16_part2(path: std::path::PathBuf) -> i32 {
    0
}

fn find_next_valve(
    valves: HashMap<String, Valve>,
    starting_node: String,
    time_remaining: usize,
) -> String {
    let len = valves.len();
    let target_valve = valves.get(&starting_node).unwrap().clone();

    let mut max = 0;
    let mut max_valve: String;

    for (v_name, v) in valves {
        let mut visited: HashSet<String> = Default::default();
        let mut q: VecDeque<(String, usize)> = Default::default();
        visited.insert(v_name);
        q.push_back((v_name, 0));

        while !q.is_empty() {
            let (valve_name, d) = q.pop_front().unwrap();
            let valve = valves.get(&valve_name).unwrap().clone();
            if valve_name == target_valve.name {
                let score = valve.score(d, time_remaining);
                if score > max {
                    max = score;
                    max_valve = valve.name;
                }
            }
            for each in valve.conected_values {
                if visited.get(&each).is_none() {
                    q.push_back((each, d + 1));
                }
            }
        }
    }
    max_valve
}

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
struct Valve {
    pub name: String,
    pub rate: usize,
    pub is_open: bool,
    pub conected_values: Vec<String>,
}

impl Valve {
    pub fn score(self, distance_to_valve: usize, time_remaining: usize) -> usize {
        (time_remaining - distance_to_valve - 1) * self.rate
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

        let rate = num.parse::<usize>().unwrap();
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
    fn test_score() {
        let valve = Valve {
            name: "BB".to_string(),
            rate: 13,
            is_open: false,
            conected_values: vec!["CC".to_string(), "AA".to_string()],
        };

        assert_eq!(valve.score(1, 30), 364);

        let valve = Valve {
            name: "CC".to_string(),
            rate: 2,
            is_open: false,
            conected_values: vec!["DD".to_string(), "BB".to_string()],
        };

        assert_eq!(valve.score(5, 28), 44);
    }

    #[test]
    fn test_find_next_valve() {
        let input = load_file_to_string_vectors("src/solution/s16/example.txt");
        let valves: Vec<Valve> = input
            .into_iter()
            .map(|line| line.parse().unwrap())
            .collect();

        assert_eq!(
            find_next_valve(valves.clone(), "AA".to_string(), 30),
            "DD".to_string()
        );
        assert_eq!(
            find_next_valve(valves.clone(), "DD".to_string(), 28),
            "BB".to_string()
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
