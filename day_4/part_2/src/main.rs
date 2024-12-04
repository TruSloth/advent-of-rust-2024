const DIAGONAL_FORWARD_SLASH: [(usize, usize); 3] = [(0, 0), (1, 1), (2, 2)];
const DIAGONAL_BACKWARD_SLASH: [(usize, usize); 3] = [(0, 2), (1, 1), (2, 0)];

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
    let result = count_x_mas(&input_grid);

    println!("{result}");
}

fn count_x_mas(input_grid: &[Vec<char>]) -> u32 {
    (0..input_grid.len())
        .map(|y| {
            (0..input_grid.get(y).unwrap().len())
                .map(|x| {
                    let forward_slash = DIAGONAL_FORWARD_SLASH
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
                    let backward_slash = DIAGONAL_BACKWARD_SLASH
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
                    let mut count = 0;
                    if (forward_slash.iter().collect::<String>() == "MAS"
                        || forward_slash.iter().rev().collect::<String>() == "MAS")
                        && (backward_slash.iter().collect::<String>() == "MAS"
                            || backward_slash.iter().rev().collect::<String>() == "MAS")
                    {
                        count += 1;
                    };
                    count
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[test]
fn forwards() {
    let input_grid = vec![
        vec!['B', 'M', 'B', 'M'],
        vec!['B', 'B', 'A', 'B'],
        vec!['B', 'S', 'B', 'S'],
        vec!['B', 'B', 'B', 'B'],
    ];
    let result = count_x_mas(&input_grid);
    assert_eq!(result, 1);
}

#[test]
fn reverse() {
    let input_grid = vec![
        vec!['B', 'S', 'B', 'S'],
        vec!['B', 'B', 'A', 'B'],
        vec!['B', 'M', 'B', 'M'],
        vec!['B', 'B', 'B', 'B'],
    ];
    let result = count_x_mas(&input_grid);
    assert_eq!(result, 1);
}
