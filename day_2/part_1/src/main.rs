fn main() {
    let stdin = std::io::stdin();

    let result = stdin.lines().map(|line| {
        if let Ok(line) = (line) {
            let report = line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let differences: Vec<i32> = report.as_slice().windows(2).map(|window| window[0] - window[1]).collect();
            let within_range = differences.iter().all(|diff| {
               diff.abs() >= 1 && diff.abs() <= 3 
            });
            
            let same_direction = differences.iter().scan(differences.first().unwrap().signum(), |state, diff| {
                match state {
                    0 => None,
                    1 => {
                        if diff.signum() == 1 {
                            Some(1)
                        } else {
                            None
                        }
                    },
                    -1 => {
                        if diff.signum() == -1 {
                            Some(-1)
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            }).count() == differences.len();
            within_range && same_direction
        } else {
            false
        }
    }).filter(|valid| *valid).count();

    println!("{result}");
}
