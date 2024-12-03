use std::{collections::HashMap, iter::zip};

fn main() {
    let stdin = std::io::stdin();
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    stdin.lines().for_each(|line| match (line) {
        Ok(line) => {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap());
            left.push(nums.next().unwrap());
            right
                .entry(nums.next().unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        Err(_) => panic!("Err"),
    });

    let result = left.iter().map(|&num| {
        let count = right.entry(num).or_default();
        num * *count
    }).sum::<u32>();

    println!("{result}");
}
