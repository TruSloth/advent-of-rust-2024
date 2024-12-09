use std::{collections::HashMap, io::Read};

#[derive(PartialEq, Eq, Debug)]
struct Map {
    map: Vec<Vec<char>>,
    antenna_locs: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut antenna_locs = HashMap::new();
        let mut map = Vec::new();

        input.lines().enumerate().for_each(|(y, line)| {
            let mut row = Vec::new();
            line.char_indices().for_each(|(x, c)| {
                row.push(c);
                if c != '.' {
                    antenna_locs
                        .entry(c)
                        .and_modify(|v: &mut Vec<(usize, usize)>| v.push((y, x)))
                        .or_insert(vec![(y, x)]);
                }
            });
            map.push(row);
        });
        Self { map, antenna_locs }
    }

    fn num_rows(&self) -> usize {
        self.map.len()
    }

    fn num_cols(&self) -> usize {
        self.map.first().unwrap().len()
    }

    #[allow(clippy::too_many_lines)]
    fn fill_antinodes(self) -> Map {
        let mut filled_map = self.map.clone();
        self.antenna_locs.values().for_each(|coordinates| {
            let num_rows = self.num_rows();
            let num_cols = self.num_cols();
            (0..coordinates.len()).for_each(|idx_1| {
                (idx_1 + 1..coordinates.len()).for_each(|idx_2| {
                    let (y1, x1) = coordinates.get(idx_1).unwrap();
                    let (y2, x2) = coordinates.get(idx_2).unwrap();
                    let y_diff = y1.abs_diff(*y2);
                    let x_diff = x1.abs_diff(*x2);

                    let (antinode_1_location, antinode_2_location) = match (y1 > y2, x1 > x2) {
                        // 2 . .
                        // . . .
                        // . . 1
                        (true, true) => {
                            let antinode_1_location = {
                                if (y1 + y_diff >= num_rows || x1 + x_diff >= num_cols) {
                                    None
                                } else {
                                    Some((y1 + y_diff, x1 + x_diff))
                                }
                            };
                            let antinode_2_location = {
                                let (y, x) = (y2.checked_sub(y_diff), x2.checked_sub(x_diff));
                                if let Some(y) = y {
                                    if let Some(x) = x {
                                        Some((y, x))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            };
                            (antinode_1_location, antinode_2_location)
                        }
                        // . . 2
                        // . . .
                        // 1 . .
                        (true, false) => {
                            let antinode_1_location = {
                                let (y, x) = (y1 + y_diff, x1.checked_sub(x_diff));
                                if let Some(x) = x {
                                    if y >= num_rows {
                                        None
                                    } else {
                                        Some((y, x))
                                    }
                                } else {
                                    None
                                }
                            };
                            let antinode_2_location = {
                                let (y, x) = (y2.checked_sub(y_diff), x2 + x_diff);
                                if let Some(y) = y {
                                    if x >= num_cols {
                                        None
                                    } else {
                                        Some((y, x))
                                    }
                                } else {
                                    None
                                }
                            };
                            (antinode_1_location, antinode_2_location)
                        }
                        // . . 1
                        // . . .
                        // 2 . .
                        (false, true) => {
                            let antinode_1_location = {
                                let (y, x) = (y1.checked_sub(y_diff), x1 + x_diff);
                                if let Some(y) = y {
                                    if x >= num_cols {
                                        None
                                    } else {
                                        Some((y, x))
                                    }
                                } else {
                                    None
                                }
                            };
                            let antinode_2_location = {
                                let (y, x) = (y2 + y_diff, x2.checked_sub(x_diff));
                                if let Some(x) = x {
                                    if y >= num_rows {
                                        None
                                    } else {
                                        Some((y, x))
                                    }
                                } else {
                                    None
                                }
                            };
                            (antinode_1_location, antinode_2_location)
                        }
                        // 1 . .
                        // . . .
                        // . . 2
                        (false, false) => {
                            let antinode_1_location = {
                                let (y, x) = (y1.checked_sub(y_diff), x1.checked_sub(x_diff));
                                if let Some(y) = y {
                                    if let Some(x) = x {
                                        Some((y, x))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            };
                            let antinode_2_location = {
                                if y2 + y_diff >= num_rows || x2 + x_diff >= num_cols {
                                    None
                                } else {
                                    Some((y2 + y_diff, x2 + x_diff))
                                }
                            };
                            (antinode_1_location, antinode_2_location)
                        }
                    };

                    if let Some((y1, x1)) = antinode_1_location {
                        let c = filled_map.get_mut(y1).unwrap().get_mut(x1).unwrap();
                        *c = '#';
                    }

                    if let Some((y2, x2)) = antinode_2_location {
                        let c = filled_map.get_mut(y2).unwrap().get_mut(x2).unwrap();
                        *c = '#';
                    }
                });
            });
        });
        Map {
            map: filled_map,
            antenna_locs: self.antenna_locs,
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    let _ = stdin.read_to_string(&mut input);
    let map = Map::new(input.as_str());
    let filled_map = map.fill_antinodes();
    let result = filled_map
        .map
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| u32::from(*c == '#'))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{result}");
}

#[test]
fn test_map_creation() {
    let input = ".aa.\n....\n....";
    let map = Map::new(input);
    let expected = Map {
        map: vec![
            vec!['.', 'a', 'a', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
        ],
        antenna_locs: [('a', vec![(0, 1), (0, 2)])].into_iter().collect(),
    };

    assert_eq!(map, expected);
}

#[test]
fn test_antinode_creation_horizontal() {
    let input = ".aa.\n....\n....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['#', 'a', 'a', '#'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
        ],
        antenna_locs: [('a', vec![(0, 1), (0, 2)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}

#[test]
fn test_antinode_creation_vertical() {
    let input = ".a..\n.a..\n....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['.', 'a', '.', '.'],
            vec!['.', 'a', '.', '.'],
            vec!['.', '#', '.', '.'],
        ],
        antenna_locs: [('a', vec![(0, 1), (1, 1)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}

#[test]
fn test_antinode_creation_diagonal() {
    let input = ".a..\n..a.\n....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['.', 'a', '.', '.'],
            vec!['.', '.', 'a', '.'],
            vec!['.', '.', '.', '#'],
        ],
        antenna_locs: [('a', vec![(0, 1), (1, 2)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}
