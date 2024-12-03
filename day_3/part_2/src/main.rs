use std::io::Read;

struct MultiplyInstruction {
    left: u32,
    right: u32,
}

enum InvalidInstruction {
    MissingComma,
    UnparsableLeftOperand,
    UnparsableRightOperand,
    MissingRightParenthesis,
}

impl MultiplyInstruction {
    fn new(input: &str) -> Result<Self, InvalidInstruction> {
        if let Some((left, right)) = input.split_once(",") {
            let left = left.parse::<u32>();
            if let Ok(left) = left {
                if let Some((right, _)) = right.split_once(")") {
                    let right = right.parse::<u32>();
                    if let Ok(right) = right {
                        Ok(Self { left, right })
                    } else {
                        Err(InvalidInstruction::UnparsableRightOperand)
                    }
                } else {
                    Err(InvalidInstruction::MissingRightParenthesis)
                }
            } else {
                Err(InvalidInstruction::UnparsableLeftOperand)
            }
        } else {
            Err(InvalidInstruction::MissingComma)
        }
    }

    fn calculate(self) -> u32 {
        self.left * self.right
    }
}

fn enabled(input: &str, prev: bool) -> bool {
    let do_idx = input.rfind("do()");
    let dont_idx = input.rfind("don't()");

    match (do_idx, dont_idx) {
        (Some(do_idx), Some(dont_idx)) => do_idx > dont_idx,
        (None, Some(_)) => false,
        (Some(_), None) => true,
        (None, None) => prev,
    }
}

fn split_input(input: &str) -> Vec<&str> {
    input.split("mul(").collect()
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();

    assert!(stdin.read_to_string(&mut input).is_ok(), "Error reading");

    let input = split_input(input.as_str());
    let result = input.iter().fold((0, true), |(acc, is_enabled), &line| {
        let value = if let Ok(instruction) = MultiplyInstruction::new(line) {
            if is_enabled {
                instruction.calculate()
            } else {
                0
            }
        } else {
            0
        };
        (acc + value, enabled(line, is_enabled))
    });
    println!("{}", result.0);
}

#[test]
fn test_split_input() {
    let result =
        split_input("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    let expected: Vec<&str> = vec![
        "x",
        "2,4)&mul[3,7]!^don't()_",
        "5,5)+",
        "32,64](",
        "11,8)undo()?",
        "8,5))",
    ];
    assert_eq!(result, expected);
}
