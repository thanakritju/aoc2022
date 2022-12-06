use std::{fs, path::Path};

#[cfg(windows)]
const TWO_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const TWO_LINE_ENDING: &'static str = "\n\n";

pub fn load_file_to_vectors<P>(file_name: P) -> Vec<Vec<i32>>
where
    P: AsRef<Path>,
{
    let str = fs::read_to_string(file_name).expect("Error in reading the file");
    let vecs = str
        .split(TWO_LINE_ENDING)
        .map(string_to_int_vector)
        .collect();
    vecs
}

pub fn load_file_split_two_lines<P>(file_name: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let str = fs::read_to_string(file_name).expect("Error in reading the file");
    let vecs = str
        .split(TWO_LINE_ENDING)
        .map(|s| String::from(s))
        .collect();
    vecs
}

pub fn load_file_to_string_vectors<P>(file_name: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let str = fs::read_to_string(file_name).expect("Error in reading the file");
    let vecs = str.lines().map(|s| String::from(s)).collect();
    vecs
}
pub fn load_file_to_string<P>(file_name: P) -> String
where
    P: AsRef<Path>,
{
    let str = fs::read_to_string(file_name).expect("Error in reading the file");
    let vecs: Vec<String> = str.lines().map(|s| String::from(s)).collect();
    vecs.get(0).expect("No data").to_string()
}

fn string_to_int_vector(s: &str) -> Vec<i32> {
    let numbers: Vec<i32> = s.lines().map(|s| s.parse().expect("parse error")).collect();
    numbers
}
