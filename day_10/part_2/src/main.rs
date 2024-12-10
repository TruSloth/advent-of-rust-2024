use std::io::Read;

struct Map {
    map: Vec<Vec<u32>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        Self { map }
    }

    fn get_height(&self, location: (usize, usize)) -> Option<u32> {
        let (y, x) = location;
        if let Some(row) = self.map.get(y) {
            row.get(x).copied()
        } else {
            None
        }
    }

    fn get_hiking_scores(self) -> Vec<Vec<u32>> {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, height)| {
                        if *height == 0 {
                            let mut score = 0;
                            let mut paths = vec![(y, x)];

                            while let Some(current_loc) = paths.pop() {
                                if let Some(height) = self.get_height(current_loc) {
                                    if height == 9 {
                                        score += 1;
                                    } else {
                                        let mut next_locations =
                                            self.get_next_step_in_trail(current_loc);
                                        paths.append(&mut next_locations);
                                    }
                                } else {
                                    panic!("Out of map!")
                                }
                            }
                            score
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn get_next_step_in_trail(&self, current_loc: (usize, usize)) -> Vec<(usize, usize)> {
        let (current_y, current_x) = current_loc;
        let mut next_locations = Vec::new();
        if let Some(current_loc_height) = self.get_height(current_loc) {
            for (dy, dx) in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let current_y = i32::try_from(current_y).unwrap();
                let y = current_y + dy;
                let current_x = i32::try_from(current_x).unwrap();
                let x = current_x + dx;

                if let Ok(y) = usize::try_from(y) {
                    if let Ok(x) = usize::try_from(x) {
                        if let Some(next_height) = self.get_height((y, x)) {
                            if next_height == current_loc_height + 1 {
                                next_locations.push((y, x));
                            }
                        }
                    }
                };
            }
        } else {
            panic!("Out of map!")
        }
        next_locations
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();

    let _ = stdin.read_to_string(&mut input);
    let map = Map::new(input.as_str());

    let hiking_scores = map.get_hiking_scores();

    let result = hiking_scores
        .iter()
        .map(|row| row.iter().sum::<u32>())
        .sum::<u32>();

    println!("{result}");
}

#[test]
fn test_map_creation() {
    let input = "0123\n4567\n8912";
    let map = Map::new(input);
    let expected = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 1, 2]];

    assert_eq!(map.map, expected);
}

#[test]
fn test_next_locations() {
    let input = "2222\n2112\n2222";
    let map = Map::new(input);
    let next_locations = map.get_next_step_in_trail((1, 2));
    let expected = vec![(0, 2), (1, 3), (2, 2)];

    assert_eq!(next_locations, expected);
}
