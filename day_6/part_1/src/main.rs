use std::io::Read;

#[derive(Debug)]
struct Guard {
    direction: GuardDirection,
    position: (usize, usize),
}

#[derive(Debug, Eq, PartialEq)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Guard {
    fn new(direction: &char, position: (usize, usize)) -> Self {
        Self {
            direction: match direction {
                'v' => GuardDirection::Down,
                '^' => GuardDirection::Up,
                '<' => GuardDirection::Left,
                '>' => GuardDirection::Right,
                _ => panic!("Unknown direction"),
            },
            position,
        }
    }

    fn next_position(&self) -> Option<(usize, usize)> {
        match self.direction {
            GuardDirection::Up => {
                if let Some(y) = self.position.0.checked_sub(1) {
                    Some((y, self.position.1))
                } else {
                    None
                }
            }
            GuardDirection::Down => Some((self.position.0 + 1, self.position.1)),
            GuardDirection::Left => {
                if let Some(x) = self.position.1.checked_sub(1) {
                    Some((self.position.0, x))
                } else {
                    None
                }
            }
            GuardDirection::Right => Some((self.position.0, self.position.1 + 1)),
        }
    }

    fn turn(&mut self) {
        match self.direction {
            GuardDirection::Up => self.direction = GuardDirection::Right,
            GuardDirection::Down => self.direction = GuardDirection::Left,
            GuardDirection::Left => self.direction = GuardDirection::Up,
            GuardDirection::Right => self.direction = GuardDirection::Down,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    guard: Guard,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut rows = Vec::new();
        let input_lines: Vec<&str> = input.lines().collect();
        let mut guard = None;
        (0..input_lines.len()).for_each(|y| {
            let line = input_lines.get(y).unwrap();
            let mut row = Vec::new();
            line.char_indices().for_each(|(x, c)| {
                row.push(c);
                match c {
                    '<' | '>' | '^' | 'v' => guard = Some(Guard::new(&c, (y, x))),
                    _ => (),
                };
            });
            rows.push(row);
        });
        Self {
            map: rows,
            guard: guard.unwrap(),
        }
    }

    fn out_of_map(&self) -> bool {
        if let Some(guard) = self.map.get(self.guard.position.0) {
            if let Some(_) = guard.get(self.guard.position.1) {
                false
            } else {
                true
            }
        } else {
            true
        }
    }

    fn walk(&mut self) -> bool {
        if self.out_of_map() {
            false
        } else {
            let next_position = self.guard.next_position();
            if let Some(next_position) = next_position {
                if let Some(pos) = self.map.get(next_position.0) {
                    if let Some(pos) = pos.get(next_position.1) {
                        match pos {
                            '.' | 'X' => {
                                let old_pos = self
                                    .map
                                    .get_mut(self.guard.position.0)
                                    .unwrap()
                                    .get_mut(self.guard.position.1)
                                    .unwrap();
                                *old_pos = 'X';
                                self.guard.position = next_position;
                                true
                            }
                            '#' => {
                                self.guard.turn();
                                true
                            }
                            _ => panic!("Unknown char in map"),
                        }
                    } else {
                        let old_pos = self
                            .map
                            .get_mut(self.guard.position.0)
                            .unwrap()
                            .get_mut(self.guard.position.1)
                            .unwrap();
                        *old_pos = 'X';
                        self.guard.position = next_position;
                        true
                    }
                } else {
                    let old_pos = self
                        .map
                        .get_mut(self.guard.position.0)
                        .unwrap()
                        .get_mut(self.guard.position.1)
                        .unwrap();
                    *old_pos = 'X';
                    self.guard.position = next_position;
                    true
                }
            } else {
                let old_pos = self
                    .map
                    .get_mut(self.guard.position.0)
                    .unwrap()
                    .get_mut(self.guard.position.1)
                    .unwrap();
                *old_pos = 'X';
                false
            }
        }
    }

    fn count_patrol_spots(&self) -> u32 {
        self.map
            .iter()
            .map(|row| row.iter().map(|c| u32::from(*c == 'X')).sum::<u32>())
            .sum::<u32>()
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    let _ = stdin.read_to_string(&mut input);

    let mut map = Map::new(input.as_str());
    while map.walk() {}
    let result = map.count_patrol_spots();

    println!("{result}");
}

#[test]
fn test_map_creation() {
    let input = "..<";
    let map = Map::new(input);
    let expected_map = vec![vec!['.', '.', '<']];

    assert_eq!(map.map, expected_map);
    assert_eq!(map.guard.position, (0, 2));
    assert_eq!(map.guard.direction, GuardDirection::Left);
}

#[test]
fn test_walk() {
    let mut map = Map {
        map: vec![vec!['.', '.', '<']],
        guard: Guard {
            direction: GuardDirection::Left,
            position: (0, 2),
        },
    };

    let can_walk = map.walk();
    assert!(can_walk);
    assert_eq!(map.guard.direction, GuardDirection::Left);
    assert_eq!(map.guard.position, (0, 1));
    assert_eq!(map.map, vec![vec!['.', '.', 'X']]);
}

#[test]
fn test_walk_into_obstacle() {
    let mut map = Map {
        map: vec![vec!['.', '#', '<']],
        guard: Guard {
            direction: GuardDirection::Left,
            position: (0, 2),
        },
    };

    let can_walk = map.walk();
    assert!(can_walk);
    assert_eq!(map.guard.direction, GuardDirection::Up);
    assert_eq!(map.guard.position, (0, 2));
    assert_eq!(map.map, vec![vec!['.', '#', '<']]);
}
