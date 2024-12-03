aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    // Parse the input into a vector of reports
    let reports: Vec<Vec<i32>> = input
    .lines()
    .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()).collect())
    .collect();

    // Function to check if a report is safe
    fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];

        // Check the difference constraint
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Check monotonicity
        if diff > 0 {
            decreasing = false;
        } else if diff < 0 {
            increasing = false;
        }
    }

    // Safe if it's entirely increasing or entirely decreasing
    increasing || decreasing
}

// Count safe reports
reports.iter().filter(|report| is_safe(report)).count()}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
