use std::collections::BinaryHeap;

fn calorie_counting(numbers: Vec<Vec<i32>>) -> i32 {
    let mut sums: BinaryHeap<i32> = numbers.iter().map(|n| n.iter().sum()).collect();
    match sums.pop() {
        Some(v) => v,
        None => 0,
    }
}

fn calorie_counting_top_three(numbers: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut sums: BinaryHeap<i32> = numbers.iter().map(|n| n.iter().sum()).collect();
    match sums.pop() {
        Some(v) => ans += v,
        None => {}
    }
    match sums.pop() {
        Some(v) => ans += v,
        None => {}
    }
    match sums.pop() {
        Some(v) => ans += v,
        None => {}
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::util::load_file::load_file_to_vectors;

    use super::*;

    #[test]
    fn test_calorie_counting() {
        let input = load_file_to_vectors(String::from(
            "src/solution/s01_calorie_counting/example.txt",
        ));
        assert_eq!(calorie_counting(input), 24000);
        let input =
            load_file_to_vectors(String::from("src/solution/s01_calorie_counting/input.txt"));
        assert_eq!(calorie_counting(input), 69626);
        let input = load_file_to_vectors(String::from(
            "src/solution/s01_calorie_counting/example.txt",
        ));
        assert_eq!(calorie_counting_top_three(input), 45000);
        let input =
            load_file_to_vectors(String::from("src/solution/s01_calorie_counting/input.txt"));
        assert_eq!(calorie_counting_top_three(input), 206780);
    }
}
