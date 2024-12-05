aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    // Parse the input into a vector of reports
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
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
    reports.iter().filter(|report| is_safe(report)).count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    // Parse the input into a vector of reports
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
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

    // Function to check if a report can be made safe by removing one level
    fn can_be_safe_with_removal(report: &[i32]) -> bool {
        if report.len() <= 2 {
            return true; // Removing any one level would make it trivially safe
        }

        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i); // Remove the level at index `i`
            if is_safe(&modified_report) {
                return true;
            }
        }

        false
    }

    // Count safe reports (originally safe or safe with one removal)
    reports
        .iter()
        .filter(|report| is_safe(report) || can_be_safe_with_removal(report))
        .count()
}
