use std::collections::VecDeque;

fn main() {
    let stdin = std::io::stdin();
    let result = stdin
        .lines()
        .map(|line| {
            if let Ok(line) = line {
                if let Some((target, operands)) = line.split_once(':') {
                    let target = target.parse::<u64>().unwrap();

                    let mut operands: VecDeque<u64> = operands
                        .split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect();

                    let acc = operands.pop_front().unwrap();
                    let operands: Vec<u64> = operands.into_iter().collect();
                    if generate_possible_combinations(operands)
                        .iter()
                        .any(|operations| {
                            let sum = operations
                                .iter()
                                .fold(acc, |acc, operation| operation.resolve(acc));
                            sum == target
                        })
                    {
                        target
                    } else {
                        0
                    }
                } else {
                    panic!("Input line with no colon")
                }
            } else {
                panic!("Unable to read line")
            }
        })
        .sum::<u64>();

    println!("{result}");
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Concat(u64),
}

impl Operation {
    fn resolve(&self, acc: u64) -> u64 {
        match self {
            Operation::Add(x) => acc + x,
            Operation::Multiply(x) => acc * x,
            Operation::Concat(x) => {
                let mut prefix = acc.to_string();
                let suffix = x.to_string();
                prefix.push_str(suffix.as_str());
                prefix.parse::<u64>().unwrap()
            }
        }
    }
}

fn generate_possible_combinations(operands: Vec<u64>) -> Vec<Vec<Operation>> {
    (0..(3_u64.pow(u32::try_from(operands.len()).unwrap())))
        .map(|i| {
            let mut combination = Vec::new();
            let mut i = i;
            (0..operands.len()).rev().for_each(|j| {
                let num = operands.get(j).unwrap();
                let val = i / (3_u64.pow(u32::try_from(j).unwrap()));
                i -= val * (3_u64.pow(u32::try_from(j).unwrap()));
                match val {
                    0 => combination.push(Operation::Add(*num)),
                    1 => combination.push(Operation::Multiply(*num)),
                    2 => combination.push(Operation::Concat(*num)),
                    _ => panic!("Should be in base 3"),
                };
            });
            combination.reverse();
            combination
        })
        .collect()
}

#[test]
fn test_add() {
    let operation = Operation::Add(5);
    let result = operation.resolve(2);
    let expected = 7;

    assert_eq!(result, expected);
}

#[test]
fn test_multiply() {
    let operation = Operation::Multiply(5);
    let result = operation.resolve(2);
    let expected = 10;

    assert_eq!(result, expected);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_generate_possible_combinations() {
    let operands = vec![1, 2, 3];
    let result: Vec<Vec<Operation>> = generate_possible_combinations(operands);
    let expected = vec![
        vec![Operation::Add(1), Operation::Add(2), Operation::Add(3)],
        vec![Operation::Multiply(1), Operation::Add(2), Operation::Add(3)],
        vec![Operation::Concat(1), Operation::Add(2), Operation::Add(3)],
        vec![Operation::Add(1), Operation::Multiply(2), Operation::Add(3)],
        vec![
            Operation::Multiply(1),
            Operation::Multiply(2),
            Operation::Add(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Multiply(2),
            Operation::Add(3),
        ],
        vec![Operation::Add(1), Operation::Concat(2), Operation::Add(3)],
        vec![
            Operation::Multiply(1),
            Operation::Concat(2),
            Operation::Add(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Concat(2),
            Operation::Add(3),
        ],
        vec![Operation::Add(1), Operation::Add(2), Operation::Multiply(3)],
        vec![
            Operation::Multiply(1),
            Operation::Add(2),
            Operation::Multiply(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Add(2),
            Operation::Multiply(3),
        ],
        vec![
            Operation::Add(1),
            Operation::Multiply(2),
            Operation::Multiply(3),
        ],
        vec![
            Operation::Multiply(1),
            Operation::Multiply(2),
            Operation::Multiply(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Multiply(2),
            Operation::Multiply(3),
        ],
        vec![
            Operation::Add(1),
            Operation::Concat(2),
            Operation::Multiply(3),
        ],
        vec![
            Operation::Multiply(1),
            Operation::Concat(2),
            Operation::Multiply(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Concat(2),
            Operation::Multiply(3),
        ],
        vec![Operation::Add(1), Operation::Add(2), Operation::Concat(3)],
        vec![
            Operation::Multiply(1),
            Operation::Add(2),
            Operation::Concat(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Add(2),
            Operation::Concat(3),
        ],
        vec![
            Operation::Add(1),
            Operation::Multiply(2),
            Operation::Concat(3),
        ],
        vec![
            Operation::Multiply(1),
            Operation::Multiply(2),
            Operation::Concat(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Multiply(2),
            Operation::Concat(3),
        ],
        vec![
            Operation::Add(1),
            Operation::Concat(2),
            Operation::Concat(3),
        ],
        vec![
            Operation::Multiply(1),
            Operation::Concat(2),
            Operation::Concat(3),
        ],
        vec![
            Operation::Concat(1),
            Operation::Concat(2),
            Operation::Concat(3),
        ],
    ];
    assert_eq!(result.len(), 27);
    assert_eq!(result, expected);
}
