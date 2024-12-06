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
            let result = updates
                .lines()
                .filter_map(|line| process_update(line, &rules))
                .map(|invalid_updates| {
                    let sorted_updates = sort_updates(invalid_updates, &rules);
                    *sorted_updates.get(sorted_updates.len().div(2)).unwrap()
                })
                .sum::<u32>();

            println!("{result}");
        } else {
            panic!("Unable to split rules and updates")
        }
    } else {
        panic!("Unable to read input")
    }
}

struct RuleItem {
    afters: HashSet<u32>,
    before: u32,
}

impl PartialEq for RuleItem {
    fn eq(&self, other: &Self) -> bool {
        (self.before == other.before) && (self.afters == other.afters)
    }
}

impl Eq for RuleItem {}

impl Ord for RuleItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (
            other.afters.contains(&self.before),
            self.afters.contains(&other.before),
        ) {
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (true, true) => panic!("Can never satisfy rule"),
            (false, false) => {
                if self.afters.len() > other.afters.len() {
                    std::cmp::Ordering::Less
                } else if self.afters.len() < other.afters.len() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for RuleItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl RuleItem {
    fn new(before: u32, afters: HashSet<u32>) -> Self {
        Self { afters, before }
    }
}

fn sort_updates(update: Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut relevant_rules = rules.clone();
    relevant_rules.retain(|k, _| update.contains(k));
    for nums in relevant_rules.values_mut() {
        nums.retain(|num| update.contains(num));
    }
    let mut relevant_rule_items: Vec<RuleItem> = relevant_rules
        .into_iter()
        .map(|(before, afters)| RuleItem::new(before, afters))
        .collect();
    relevant_rule_items.sort_unstable();
    let mut sorted_update: Vec<u32> = relevant_rule_items
        .into_iter()
        .map(|item| item.before)
        .collect();
    for num in update {
        if !sorted_update.contains(&num) {
            sorted_update.push(num);
        }
    }
    sorted_update
}

fn process_update(update: &str, rules: &HashMap<u32, HashSet<u32>>) -> Option<Vec<u32>> {
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

    if !valid {
        Some(nums)
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
    assert_eq!(result, None);
}

#[test]
fn test_invalid_update() {
    let update = "75,29,13";
    let mut rules = HashMap::new();
    rules.insert(13, iter::once(29).collect());

    let result = process_update(update, &rules);
    assert_eq!(result, Some(vec![75, 29, 13]));
}

#[test]
fn test_sort_invalid_update() {
    let invalid_update = vec![75, 29, 13];
    let mut rules = HashMap::new();
    rules.insert(13, iter::once(29).collect());
    rules.insert(75, iter::once(13).collect());

    let result = sort_updates(invalid_update, &rules);
    assert_eq!(result, vec![75, 13, 29]);
}
