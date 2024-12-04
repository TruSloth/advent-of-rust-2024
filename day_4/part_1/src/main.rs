const VERTICAL: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const HORIZONTAL: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const DIAGONAL_BOTTOM_RIGHT: [(usize, usize); 4] = [(0, 0), (1, 1), (2, 2), (3, 3)];
const DIAGONAL_BOTTOM_LEFT: [(i32, i32); 4] = [(0, 0), (1, -1), (2, -2), (3, -3)];

fn main() {
    let stdin = std::io::stdin();

    let input_grid: Vec<Vec<char>> = stdin
        .lines()
        .map(|line| {
            if let Ok(line) = line {
                line.chars().collect::<Vec<char>>()
            } else {
                panic!("Unable to process");
            }
        })
        .collect();
    let result = count_xmas(&input_grid);

    println!("{result}");
}

fn count_xmas(input_grid: &[Vec<char>]) -> u32 {
    (0..input_grid.len())
        .map(|y| {
            (0..input_grid.get(y).unwrap().len())
                .map(|x| {
                    let vertical = VERTICAL
                        .iter()
                        .filter_map(|(offset_y, offset_x)| {
                            if let Some(row) = input_grid.get(y + offset_y) {
                                if let Some(&c) = row.get(x + offset_x) {
                                    Some(c)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<char>>();
                    let horizontal = HORIZONTAL
                        .iter()
                        .filter_map(|(offset_y, offset_x)| {
                            if let Some(row) = input_grid.get(y + offset_y) {
                                if let Some(&c) = row.get(x + offset_x) {
                                    Some(c)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<char>>();
                    let diagonal_bottom_right = DIAGONAL_BOTTOM_RIGHT
                        .iter()
                        .filter_map(|(offset_y, offset_x)| {
                            if let Some(row) = input_grid.get(y + offset_y) {
                                if let Some(&c) = row.get(x + offset_x) {
                                    Some(c)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<char>>();
                    let diagonal_bottom_left = DIAGONAL_BOTTOM_LEFT
                        .iter()
                        .filter_map(|(offset_y, offset_x)| {
                            let y = usize::try_from(i32::try_from(y).unwrap() + offset_y);
                            let x = usize::try_from(i32::try_from(x).unwrap() + offset_x);

                            if y.is_err() || x.is_err() {
                                return None;
                            }

                            if let Some(row) = input_grid.get(y.unwrap()) {
                                if let Some(&c) = row.get(x.unwrap()) {
                                    Some(c)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<char>>();
                    let mut count = 0;
                    if vertical.iter().collect::<String>() == "XMAS"
                        || vertical.iter().rev().collect::<String>() == "XMAS"
                    {
                        println!("({y},{x}) - vertical");
                        count += 1;
                    };
                    if horizontal.iter().collect::<String>() == "XMAS"
                        || horizontal.iter().rev().collect::<String>() == "XMAS"
                    {
                        println!("({y},{x}) - horizontal");
                        count += 1;
                    };
                    if diagonal_bottom_right.iter().collect::<String>() == "XMAS"
                        || diagonal_bottom_right.iter().rev().collect::<String>() == "XMAS"
                    {
                        println!("({y},{x}) - diagonal bottom right");
                        count += 1;
                    };
                    if diagonal_bottom_left.iter().collect::<String>() == "XMAS"
                        || diagonal_bottom_left.iter().rev().collect::<String>() == "XMAS"
                    {
                        println!("({y},{x}) - diagonal bottom left");
                        count += 1;
                    };
                    count
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[test]
fn horizontal() {
    let input_grid = vec![vec!['X', 'M', 'A', 'S']];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn horizontal_reverse() {
    let input_grid = vec![vec!['S', 'A', 'M', 'X']];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn vertical() {
    let input_grid = vec![vec!['X'], vec!['M'], vec!['A'], vec!['S']];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn vertical_reverse() {
    let input_grid = vec![vec!['S'], vec!['A'], vec!['M'], vec!['X']];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn diagonal_bottom_right() {
    let input_grid = vec![
        vec!['X', 'B', 'B', 'B'],
        vec!['B', 'M', 'B', 'B'],
        vec!['B', 'B', 'A', 'B'],
        vec!['B', 'B', 'B', 'S'],
    ];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn diagonal_bottom_right_reverse() {
    let input_grid = vec![
        vec!['S', 'B', 'B', 'B'],
        vec!['B', 'A', 'B', 'B'],
        vec!['B', 'B', 'M', 'B'],
        vec!['B', 'B', 'B', 'X'],
    ];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn diagonal_bottom_left() {
    let input_grid = vec![
        vec!['B', 'B', 'B', 'X'],
        vec!['B', 'B', 'M', 'B'],
        vec!['B', 'A', 'B', 'B'],
        vec!['S', 'B', 'B', 'B'],
    ];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn diagonal_bottom_left_reverse() {
    let input_grid = vec![
        vec!['B', 'B', 'B', 'S'],
        vec!['B', 'B', 'A', 'B'],
        vec!['B', 'M', 'B', 'B'],
        vec!['X', 'B', 'B', 'B'],
    ];
    let result = count_xmas(&input_grid);
    assert_eq!(result, 1);
}
