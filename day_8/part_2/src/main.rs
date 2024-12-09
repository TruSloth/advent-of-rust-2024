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

                    let y1 = u16::try_from(*y1).unwrap();
                    let y2 = u16::try_from(*y2).unwrap();
                    let x1 = u16::try_from(*x1).unwrap();
                    let x2 = u16::try_from(*x2).unwrap();

                    let gradient =
                        { (f64::from(y1) - f64::from(y2)) / (f64::from(x1) - f64::from(x2)) };

                    let constant = f64::from(y1) - (gradient * f64::from(x1));

                    self.map.iter().enumerate().for_each(|(y, row)| {
                        row.iter().enumerate().for_each(|(x, c)| {
                            // Check if it lies on the line
                            if gradient.is_infinite() {
                                if x == usize::from(x1) {
                                    let c = filled_map.get_mut(y).unwrap().get_mut(x).unwrap();
                                    *c = '#';
                                }
                            } else {
                                let x_u16 = u16::try_from(x).unwrap();
                                let expected_y = (gradient * f64::from(x_u16)) + constant;
                                let y_u16 = u16::try_from(y).unwrap();

                                if (f64::from(y_u16) - expected_y).abs() < 0.01 {
                                    let c = filled_map.get_mut(y).unwrap().get_mut(x).unwrap();
                                    *c = '#';
                                };
                            }
                        });
                    });
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
        .map(|row| row.iter().map(|c| u32::from(*c == '#')).sum::<u32>())
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
    let input = ".aa..\n.....\n.....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ],
        antenna_locs: [('a', vec![(0, 1), (0, 2)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}

#[test]
fn test_antinode_creation_vertical() {
    let input = ".a..\n.a..\n....\n....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '#', '.', '.'],
            vec!['.', '#', '.', '.'],
            vec!['.', '#', '.', '.'],
        ],
        antenna_locs: [('a', vec![(0, 1), (1, 1)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}

#[test]
fn test_antinode_creation_diagonal() {
    let input = ".a...\n..a..\n.....\n.....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['.', '#', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '#'],
        ],
        antenna_locs: [('a', vec![(0, 1), (1, 2)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}

#[test]
fn test_antinode_creation_diagonal_inbetween() {
    let input = ".....\n.a...\n.....\n...a.\n.....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['#', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '#'],
        ],
        antenna_locs: [('a', vec![(1, 1), (3, 3)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}

#[test]
fn test_antinode_creation_horizontal_inbetween() {
    let input = ".....\n.....\n.a.a.\n.....\n.....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['#', '#', '#', '#', '#'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ],
        antenna_locs: [('a', vec![(2, 1), (2, 3)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}

#[test]
fn test_antinode_single_antenna() {
    let input = ".....\n.....\n..a..\n.....\n.....";
    let map = Map::new(input);
    let filled_map = map.fill_antinodes();
    let expected = Map {
        map: vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', 'a', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ],
        antenna_locs: [('a', vec![(2, 2)])].into_iter().collect(),
    };

    assert_eq!(filled_map, expected);
}
