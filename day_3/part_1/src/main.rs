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

fn split_input(input: &str) -> Vec<&str> {
    input.split("mul(").collect()
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();

    assert!(stdin.read_to_string(&mut input).is_ok(), "Error reading");

    let input = split_input(input.as_str());
    let result = input.iter().filter_map(|&line| {
        if let Ok(instruction) = MultiplyInstruction::new(line) {
            Some(instruction.calculate())
        } else {
            None
        }
    }).sum::<u32>();
    println!("{result}");
}

#[test]
fn test_split_input() {
    let result =
        split_input("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)");
    let mut expected: Vec<&str> = Vec::new();
    expected.push("x");
    expected.push("2,4)%&mul[3,7]!@^do_not_");
    expected.push("5,5)+");
    expected.push("32,64]then(");
    expected.push("11,8)");
    expected.push("8,5)");
    assert_eq!(result, expected);
}
