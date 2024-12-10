use std::io::Read;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Block {
    val: Option<u32>,
    size: u8,
}

impl Block {
    fn new(size: u8, val: Option<u32>) -> Self {
        Self { size, val }
    }
}

fn shift_blocks(blocks: &mut Vec<Block>) {
    let mut right_ptr = blocks.len() - 1;

    loop {
        // Advance right_ptr until stored space
        while blocks.get(right_ptr).unwrap().val.is_none() {
            right_ptr -= 1;
        }

        let block_to_shift = blocks.get(right_ptr).unwrap().clone();
        let required_size = block_to_shift.size;
        let shifted_val = block_to_shift.val;
        {
            // Searchable blocks
            let searchable_blocks = blocks.get_mut(0..right_ptr).unwrap();

            if searchable_blocks.is_empty() {
                break;
            }

            // Advance left_ptr until free space
            if let Some((idx, viable_block)) = searchable_blocks
                .iter_mut()
                .enumerate()
                .find(|(_, block)| block.val.is_none() && block.size >= required_size)
            {
                if viable_block.size == required_size {
                    *viable_block = Block::new(required_size, shifted_val);
                    *blocks.get_mut(right_ptr).unwrap() = Block::new(required_size, None);
                } else {
                    *viable_block = Block::new(viable_block.size - required_size, None);
                    blocks.insert(idx, Block::new(required_size, shifted_val));
                    *blocks.get_mut(right_ptr + 1).unwrap() = Block::new(required_size, None);
                }
            } else {
                right_ptr -= 1;
            }
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    let _ = stdin.read_to_string(&mut input);

    let mut blocks: Vec<Block> = input
        .trim()
        .char_indices()
        .map(|(idx, c)| {
            let size: u8 = c.to_digit(10).unwrap().try_into().unwrap();
            if idx % 2 == 0 {
                let val = u32::try_from(idx / 2).unwrap();
                Block::new(size, Some(val))
            } else {
                Block::new(size, None)
            }
        })
        .collect();

    shift_blocks(&mut blocks);

    let (_, result) = blocks.iter().fold((0, 0), |(offset, acc), block| {
        if let Some(val) = block.val {
            let checksum = (0..block.size)
                .map(|idx| (u64::from(idx) + offset) * u64::from(val))
                .sum::<u64>();
            (offset + u64::from(block.size), acc + checksum)
        } else {
            (offset + u64::from(block.size), acc)
        }
    });

    println!("{result}");
}

#[test]
fn test_create_block() {
    let block = Block::new(3, Some(9));
    assert_eq!(
        block,
        Block {
            size: 3,
            val: Some(9)
        }
    );
}

#[test]
fn test_shift_blocks() {
    let mut blocks = vec![
        Block::new(2, Some(0)),
        Block::new(3, None),
        Block::new(3, Some(1)),
    ];
    shift_blocks(&mut blocks);
    let b = [
        Block {
            val: Some(0),
            size: 2,
        },
        Block {
            val: Some(9),
            size: 2,
        },
        Block { val: None, size: 1 },
        Block {
            val: Some(1),
            size: 3,
        },
        Block { val: None, size: 3 },
        Block {
            val: Some(2),
            size: 1,
        },
        Block { val: None, size: 3 },
        Block {
            val: Some(3),
            size: 3,
        },
        Block { val: None, size: 1 },
        Block {
            val: Some(4),
            size: 2,
        },
        Block { val: None, size: 1 },
        Block {
            val: Some(5),
            size: 4,
        },
        Block { val: None, size: 1 },
        Block {
            val: Some(6),
            size: 4,
        },
        Block { val: None, size: 1 },
        Block {
            val: Some(7),
            size: 3,
        },
        Block { val: None, size: 1 },
        Block {
            val: Some(8),
            size: 4,
        },
        Block { val: None, size: 0 },
        Block { val: None, size: 2 },
    ];

    assert_eq!(
        blocks,
        vec![
            Block::new(2, Some(0)),
            Block::new(3, Some(1)),
            Block::new(3, None)
        ]
    );
}
