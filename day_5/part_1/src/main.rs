use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::iter;
use std::ops::Div;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();

    if stdin.read_to_string(&mut input).is_ok() {
        if let Some((rules, updates)) = input.split_once("\n\n") {
            let rules = build_rules(rules);
            let result = updates.lines().filter_map(|line| {
                process_update(line, &rules)
            }).sum::<u32>();

            println!("{result}");
        } else {
            panic!("Unable to split rules and updates")
        }
    } else {
        panic!("Unable to read input")
    }
}

fn process_update(update: &str, rules: &HashMap<u32, HashSet<u32>>) -> Option<u32> {
    let nums: Vec<u32> = update
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let mut invalid_nums: HashSet<u32> = HashSet::new();
    let mut valid = true;

    nums.iter().rev().for_each(|num| {
        if invalid_nums.contains(num) {
            valid = false;
        } else if let Some(new_invalid_nums) = rules.get(num) {
            invalid_nums.extend(new_invalid_nums);
        }
    });

    if valid {
        let mid_idx = nums.len().div(2);
        Some(*nums.get(mid_idx).unwrap())
    } else {
        None
    }
}

fn build_rules(rules_input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut rules = HashMap::new();
    rules_input
        .lines()
        .map(|line| {
            if let Some((left, right)) = line.split_once("|") {
                let left = left.parse::<u32>().unwrap();
                let right = right.parse::<u32>().unwrap();
                (left, right)
            } else {
                panic!("Invalid rule")
            }
        })
        .for_each(|(left, right)| {
            rules
                .entry(left)
                .and_modify(|afters: &mut HashSet<u32>| {
                    afters.insert(right);
                })
                .or_insert(iter::once(right).collect());
        });
    rules
}

#[test]
fn test_build_rules() {
    let rules_input = "47|53\n97|13";
    let rules = build_rules(rules_input);
    let mut expected: HashMap<u32, HashSet<u32>> = HashMap::new();
    expected.insert(47, iter::once(53).collect());
    expected.insert(97, iter::once(13).collect());
    assert_eq!(rules, expected);
}

#[test]
fn test_build_rules_with_multiple_afters() {
    let rules_input = "47|53\n47|13";
    let rules: HashMap<u32, HashSet<u32>> = build_rules(rules_input);
    let mut expected = HashMap::new();
    expected.insert(47, [53, 13].into_iter().collect());
    assert_eq!(rules, expected);
}

#[test]
fn test_simple_update() {
    let update = "75,29,13";
    let rules = HashMap::new();

    let result = process_update(update, &rules);
    assert_eq!(result, Some(29));
}

#[test]
fn test_invalid_update() {
    let update = "75,29,13";
    let mut rules = HashMap::new();
    rules.insert(13, iter::once(29).collect());

    let result = process_update(update, &rules);
    assert_eq!(result, None);
}
