use std::collections::BinaryHeap;

use regex::Regex;

use crate::utils::load_file::load_file_split_two_lines;

pub fn solution_day11_part1(path: std::path::PathBuf) -> usize {
    let input = load_file_split_two_lines(path);
    find_answer(input, 20, true)
}

pub fn solution_day11_part2(path: std::path::PathBuf) -> usize {
    let input = load_file_split_two_lines(path);
    find_answer(input, 10000, false)
}

fn find_answer(input: Vec<String>, rounds: usize, divided_by_three: bool) -> usize {
    let mut monkeys = parse_input_vec(input);
    for r in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).expect("no data");
            let mut tmp: Vec<(usize, usize)> = vec![];
            for num in &monkey.items {
                let mut out_num: usize = match monkey.operand {
                    0 => num * num,
                    operand => match monkey.operation {
                        Operation::Multiply => num * operand,
                        Operation::Add => num + operand,
                    },
                };
                if divided_by_three {
                    out_num = out_num / 3
                }
                let target_monkey_id = match out_num % monkey.divisor {
                    0 => monkey.monkey_if_true,
                    _ => monkey.monkey_if_false,
                };
                tmp.push((out_num, target_monkey_id));
            }
            monkey.clear();
            for (out_num, target_monkey_id) in tmp {
                let target_monkey = monkeys.get_mut(target_monkey_id).expect("no data");
                // target_monkey.add_item(out_num);
                match divided_by_three {
                    true => target_monkey.add_item(out_num),
                    false => target_monkey.add_item(
                        out_num - ((out_num / target_monkey.divisor) * target_monkey.divisor),
                    ),
                }
            }
        }
        for m in &monkeys {
            println!("round: {} monkey:{} items:{:?}", r + 1, m.id, m.items)
        }
    }
    let mut lens: BinaryHeap<usize> = monkeys
        .iter()
        .map(|m| {
            println!("{}", m.inspect_times);
            m.inspect_times
        })
        .collect();
    let mut ans = 1;
    match lens.pop() {
        Some(v) => ans *= v,
        None => {}
    }
    match lens.pop() {
        Some(v) => ans *= v,
        None => {}
    }
    ans
}

fn parse_input_vec(input: Vec<String>) -> Vec<Monkey> {
    input.iter().map(|s| parse_monkey(s.to_string())).collect()
}

fn parse_monkey(input: String) -> Monkey {
    let re = Regex::new(
        r"Monkey ([\d]):
\s+Starting items: ([\d\\,\\ ]+)
\s+Operation: new = old (\+|\*) (old|[\d]+)
\s+Test: divisible by ([\d]+)
\s+If true: throw to monkey ([\d])
\s+If false: throw to monkey ([\d])",
    )
    .unwrap();
    let caps = re.captures(input.as_str()).unwrap();
    let id = caps.get(1).map_or("0", |m| m.as_str());
    let starting_items = caps.get(2).map_or("0", |m| m.as_str());
    let operation = caps.get(3).map_or("0", |m| m.as_str());
    let operand = caps.get(4).map_or("0", |m| m.as_str());
    let divisor = caps.get(5).map_or("0", |m| m.as_str());
    let monkey_if_true = caps.get(6).map_or("0", |m| m.as_str());
    let monkey_if_false = caps.get(7).map_or("0", |m| m.as_str());
    Monkey {
        id: id.parse().expect("parse error"),
        items: starting_items
            .split(", ")
            .map(|s| s.parse().expect("parse error"))
            .collect(),
        operation: match operation {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            o => panic!("not regonized operation: {}", o),
        },
        operand: match operand {
            "old" => 0,
            v => v.parse().expect("parse error"),
        },
        divisor: divisor.parse().expect("parse error"),
        monkey_if_true: monkey_if_true.parse().expect("parse error"),
        monkey_if_false: monkey_if_false.parse().expect("parse error"),
        inspect_times: 0,
    }
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    operand: usize,
    divisor: usize,
    monkey_if_true: usize,
    monkey_if_false: usize,
    inspect_times: usize,
}

impl Monkey {
    pub fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn clear(&mut self) {
        self.inspect_times += self.items.len();
        self.items = vec![]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Multiply,
    Add,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_monkey() {
        let input = "Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
    Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3"
            .to_string();
        let monkey = parse_monkey(input);
        assert_eq!(monkey.id, 2);
        assert_eq!(monkey.items, vec![79, 60, 97]);
        assert_eq!(monkey.operation, Operation::Multiply);
        assert_eq!(monkey.operand, 0);
        assert_eq!(monkey.divisor, 13);
        assert_eq!(monkey.monkey_if_true, 1);
        assert_eq!(monkey.monkey_if_false, 3);
        let input = "
Monkey 0:
  Starting items: 89, 94, 94, 67
  Operation: new = old + 2
  Test: divisible by 19
    If true: throw to monkey 7
    If false: throw to monkey 0"
            .to_string();
        let monkey = parse_monkey(input);
        assert_eq!(monkey.id, 0);
        assert_eq!(monkey.items, vec![89, 94, 94, 67]);
        assert_eq!(monkey.operation, Operation::Add);
        assert_eq!(monkey.operand, 2);
        assert_eq!(monkey.divisor, 19);
        assert_eq!(monkey.monkey_if_true, 7);
        assert_eq!(monkey.monkey_if_false, 0);
    }

    #[test]
    fn test_find_answer() {
        let input = load_file_split_two_lines("src/solution/s11/example.txt");
        let ans = find_answer(input, 20, false);
        assert_eq!(ans, 99 * 103);
        let input = load_file_split_two_lines("src/solution/s11/example.txt");
        let ans = find_answer(input, 1000, false);
        assert_eq!(ans, 5204 * 5192);
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day11_part1(PathBuf::from("src/solution/s11/example.txt")),
            10605
        );
        assert_eq!(
            solution_day11_part1(PathBuf::from("src/solution/s11/input.txt")),
            55944
        );
        // assert_eq!(
        //     solution_day11_part2(PathBuf::from("src/solution/s11/example.txt")),
        //     2713310158
        // );
        // assert_eq!(
        //     solution_day11_part2(PathBuf::from("src/solution/s11/input.txt")),
        //     0
        // );
    }
}
