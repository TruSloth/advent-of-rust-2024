use std::collections::VecDeque;

fn main() {
    let stdin = std::io::stdin();
    let result = stdin.lines().map(|line| {
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
    }).sum::<u64>();

    println!("{result}");
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
}

impl Operation {
    fn resolve(&self, acc: u64) -> u64 {
        match self {
            Operation::Add(x) => acc + x,
            Operation::Multiply(x) => acc * x,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct UndoOperation {
    operation: Operation,
    val: u64,
}

impl UndoOperation {
    fn undo(self) -> u64 {
        match self.operation {
            Operation::Add(x) => self.val - x,
            Operation::Multiply(x) => self.val / x,
        }
    }
}

fn generate_possible_combinations(operands: Vec<u64>) -> Vec<Vec<Operation>> {
    (0..(1 << operands.len()))
        .map(|i| {
            let mut combination = Vec::new();
            (0..operands.len()).for_each(|j| {
                let num = operands.get(j).unwrap();
                // jth position has 1 set
                if (i & (1 << j)) == (1 << j) {
                    combination.push(Operation::Multiply(*num));
                } else {
                    combination.push(Operation::Add(*num));
                }
            });
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
fn test_undo_add() {
    let undo_operation = UndoOperation {
        operation: Operation::Add(5),
        val: 7,
    };
    let result = undo_operation.undo();

    assert_eq!(result, 2);
}

#[test]
fn test_undo_multiply() {
    let undo_operation = UndoOperation {
        operation: Operation::Multiply(5),
        val: 10,
    };
    let result = undo_operation.undo();

    assert_eq!(result, 2);
}

#[test]
fn test_generate_possible_combinations() {
    let operands = vec![1, 2, 3];
    let result: Vec<Vec<Operation>> = generate_possible_combinations(operands);
    let expected = vec![
        vec![Operation::Add(1), Operation::Add(2), Operation::Add(3)],
        vec![Operation::Multiply(1), Operation::Add(2), Operation::Add(3)],
        vec![Operation::Add(1), Operation::Multiply(2), Operation::Add(3)],
        vec![
            Operation::Multiply(1),
            Operation::Multiply(2),
            Operation::Add(3),
        ],
        vec![Operation::Add(1), Operation::Add(2), Operation::Multiply(3)],
        vec![
            Operation::Multiply(1),
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
    ];
    assert_eq!(result, expected);
}
