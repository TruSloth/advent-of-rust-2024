fn main() {
    let stdin = std::io::stdin();

    let result = stdin
        .lines()
        .map(|line| {
            if let Ok(line) = (line) {
                valid_report(&line)
            } else {
                false
            }
        })
        .filter(|valid| *valid)
        .count();

    println!("{result}");
}

fn valid_report(report: &str) -> bool {
    let report: Vec<u32> = report
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let differences: Vec<i64> = report
        .as_slice()
        .windows(2)
        .map(|window| i64::from(window[0]) - i64::from(window[1]))
        .collect();

    // Check if the report is valid as is
    let within_range = differences.iter().all(|diff| diff.abs() <= 3);
    let same_direction = differences.iter().all(|diff| diff.signum() == 1)
        || differences.iter().all(|diff| diff.signum() == -1);
    if (within_range && same_direction) {
        true
    } else {
        (0..report.len()).map(|i| {
            let mut altered_report = report.clone();
            altered_report.remove(i);

            let differences: Vec<i64> = altered_report
                .as_slice()
                .windows(2)
                .map(|window| i64::from(window[0]) - i64::from(window[1]))
                .collect();

            let within_range = differences.iter().all(|diff| diff.abs() <= 3);
            let same_direction = differences.iter().all(|diff| diff.signum() == 1)
                || differences.iter().all(|diff| diff.signum() == -1);
            within_range && same_direction
        }).any(|result| result)
    }
}

#[test]
fn simple_valid_report() {
    let result = valid_report("7 6 4 2 1");
    assert!(result);
}

#[test]
fn invalid_report() {
    let result = valid_report("1 2 7 8 9");
    assert!(!result);
}

#[test]
fn invalid_report_2() {
    let result = valid_report("9 7 6 2 1");
    assert!(!result);
}

#[test]
fn valid_report_after_removal() {
    let result = valid_report("1 3 2 4 5");
    assert!(result);
}

#[test]
fn valid_report_after_removal_at_end() {
    let result = valid_report("9 6 5 7");
    assert!(result);
}

#[test]
fn valid_report_after_removal_at_start() {
    let result = valid_report("4 4 3 2 1");
    assert!(result);
}
