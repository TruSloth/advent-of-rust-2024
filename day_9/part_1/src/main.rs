use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    let _ = stdin.read_to_string(&mut input);

    let mut blocks: Vec<String> = input
        .trim()
        .char_indices()
        .flat_map(|(idx, c)| {
            let size: u8 = c.to_digit(10).unwrap().try_into().unwrap();
            if idx % 2 == 0 {
                let val = u32::try_from(idx / 2).unwrap();
                create_block(usize::from(size), val.to_string())
            } else {
                create_block(usize::from(size), ".".to_string())
            }
        })
        .collect();

    let mut left_ptr = 0;
    let mut right_ptr = blocks.len() - 1;

    loop {
        // Advance left_ptr until free space
        while *blocks.get_mut(left_ptr).unwrap() != "." {
            left_ptr += 1;
        }

        // Advance right_ptr until stored space
        while *blocks.get_mut(right_ptr).unwrap() == "." {
            right_ptr -= 1;
        }

        if left_ptr >= right_ptr {
            break;
        }

        // Swap stored and free space
        let val = blocks.get(right_ptr).unwrap();
        *blocks.get_mut(left_ptr).unwrap() = val.to_string();
        *blocks.get_mut(right_ptr).unwrap() = ".".to_string();
    }

    let result = blocks
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            if s != "." {
                let val = s.parse::<usize>().unwrap();
                idx * val
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("{result}");
}

fn create_block(size: usize, val: String) -> Vec<String> {
    (0..size).map(|_| val.clone()).collect()
}

#[test]
fn test_create_block() {
    let block = create_block(3, "9".to_string());
    assert_eq!(block, vec!["9", "9", "9"]);
}

#[test]
fn test_create_empty_space() {
    let block = create_block(5, ".".to_string());
    assert_eq!(block, vec![".", ".", ".", ".", "."]);
}
