use std::iter::zip;

fn main() {
    let stdin = std::io::stdin();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    stdin.lines().for_each(|line| match (line) {
        Ok(line) => {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap());
            left.push(nums.next().unwrap());
            right.push(nums.next().unwrap());
        }
        Err(_) => panic!("Err"),
    });

    left.sort_unstable();
    right.sort_unstable();

    let result = zip(left, right)
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u32>();

    println!("{result}");
}
