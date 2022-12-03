use std::fs;

pub fn load_file_to_vectors(file_name: String) -> Vec<Vec<i32>> {
    let str = fs::read_to_string(file_name).expect("Error in reading the file");
    let vecs = str.split("\n\n").map(string_to_int_vector).collect();
    vecs
}

pub fn load_file_to_string_vectors(file_name: String) -> Vec<String> {
    let str = fs::read_to_string(file_name).expect("Error in reading the file");
    let vecs = str.lines().map(|s| String::from(s)).collect();
    vecs
}

fn string_to_int_vector(s: &str) -> Vec<i32> {
    let numbers: Vec<i32> = s.lines().map(|s| s.parse().expect("parse error")).collect();
    numbers
}
