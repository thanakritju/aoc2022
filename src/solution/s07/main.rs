use std::{cell::RefCell, rc::Rc};

use crate::utils::load_file::load_file_to_string_vectors;

pub fn solution_day7_part1(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let root = parse_input(input);
    let mut stack: Vec<Rc<RefCell<Node>>> = Default::default();
    let mut current = Rc::clone(&root);
    let mut ans = 0;
    stack.push(current);
    while !stack.is_empty() {
        current = stack.pop().expect("no data");
        let s = current.borrow().size();
        if s < 100000 && current.borrow().is_directory {
            ans += s
        }

        for n in &current.borrow().children {
            let current_clone = Rc::clone(&n);
            stack.push(current_clone)
        }
    }
    ans
}

pub fn solution_day7_part2(path: std::path::PathBuf) -> i32 {
    let input = load_file_to_string_vectors(path);
    let root = parse_input(input);
    let mut stack: Vec<Rc<RefCell<Node>>> = Default::default();
    let mut current = Rc::clone(&root);

    let currently_used = root.borrow().size();
    let need_more_space = currently_used - 40000000;
    let mut ans = 70000000;
    stack.push(current);
    while !stack.is_empty() {
        current = stack.pop().expect("no data");
        let s = current.borrow().size();
        if s > need_more_space && s < ans && current.borrow().is_directory {
            ans = s
        }

        for n in &current.borrow().children {
            let current_clone = Rc::clone(&n);
            stack.push(current_clone)
        }
    }
    ans
}

fn parse_input(commands: Vec<String>) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new(true, 0, "/".to_string())));
    let mut current = Rc::clone(&root);
    for command in commands {
        if command.starts_with("$ cd") {
            let out: Vec<String> = command.split("$ cd").map(|s| s.to_string()).collect();
            let arg = out.get(1).expect("No data").trim();
            if arg.eq("/") {
                continue;
            }
            if arg.eq("..") {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                continue;
            } else {
                let mut current_clone = Rc::clone(&current);
                for n in &current.borrow().children {
                    if n.borrow().name.eq(arg) {
                        current_clone = Rc::clone(&n);
                    }
                }
                current = current_clone;
                continue;
            }
        }
        if command.starts_with("$ ls") {
            continue;
        }
        if command.starts_with("dir ") {
            let out: Vec<String> = command.split("dir ").map(|s| s.to_string()).collect();
            let folder_name = out.get(1).expect("No data");

            let child = Rc::new(RefCell::new(Node::new(true, 0, folder_name.to_string())));
            let mut mut_child = child.borrow_mut();
            mut_child.parent = Some(Rc::clone(&current));
            current.borrow_mut().add_child(Rc::clone(&child));
            continue;
        } else {
            let out: Vec<String> = command.split(" ").map(|s| s.to_string()).collect();
            let size: i32 = out.get(0).expect("No data").parse().expect("parse error");
            let file_name: String = out.get(1).expect("No data").parse().expect("parse error");

            let child = Rc::new(RefCell::new(Node::new(false, size, file_name.to_string())));
            let mut mut_child = child.borrow_mut();
            mut_child.parent = Some(Rc::clone(&current));
            current.borrow_mut().add_child(Rc::clone(&child));
            continue;
        }
    }
    root
}

struct Node {
    is_directory: bool,
    children: Vec<Rc<RefCell<Node>>>,
    size: i32,
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(is_dir: bool, size: i32, name: String) -> Node {
        return Node {
            is_directory: is_dir,
            children: vec![],
            size: size,
            name: name.to_string(),
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Node>>) {
        self.children.push(new_node);
    }

    pub fn print(&self, height: usize) -> String {
        if !self.is_directory {
            return "  ".repeat(height) + &format!("- {} (file, size={})\n", self.name, self.size);
        } else {
            return "  ".repeat(height)
                + &format!("- {} (dir)\n", self.name)
                + &self
                    .children
                    .iter()
                    .map(|tn| tn.borrow().print(height + 1))
                    .collect::<Vec<String>>()
                    .join("");
        }
    }

    pub fn size(&self) -> i32 {
        if !self.is_directory {
            return self.size;
        } else {
            return self.children.iter().map(|tn| tn.borrow().size()).sum();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_size() {
        let input = load_file_to_string_vectors("src/solution/s07/example.txt");
        let root = parse_input(input);
        assert_eq!(root.borrow().size(), 48381165);
        let input = load_file_to_string_vectors("src/solution/s07/input.txt");
        let root = parse_input(input);
        assert_eq!(root.borrow().size(), 46552309);
    }

    #[test]
    fn test_print() {
        let input = load_file_to_string_vectors("src/solution/s07/example.txt");
        let root = parse_input(input);
        assert_eq!(
            root.borrow().print(0),
            "- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
"
        );
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            solution_day7_part1(PathBuf::from("src/solution/s07/example.txt")),
            95437
        );
        assert_eq!(
            solution_day7_part1(PathBuf::from("src/solution/s07/input.txt")),
            1723892
        );
        assert_eq!(
            solution_day7_part2(PathBuf::from("src/solution/s07/example.txt")),
            24933642
        );
        assert_eq!(
            solution_day7_part2(PathBuf::from("src/solution/s07/input.txt")),
            8474158
        );
    }
}
