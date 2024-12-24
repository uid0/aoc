aoc::parts!(1, 2);

use std::collections::HashSet;

pub fn solve_part_1(numbers: &[i64]) -> i64 {
    use std::collections::HashSet;
    let mut seen = HashSet::new();

    for &num in numbers {
        let complement = 2020 - num;
        if seen.contains(&complement) {
            return num * complement;
        }
        seen.insert(num);
    }

    // If no pair is found that sums to 2020, return 0 or handle as needed.
    0
}

pub fn solve_part_2(numbers: &[i64]) -> i64 {
    // Insert all numbers into a HashSet for O(1) membership checks
    let set: HashSet<_> = numbers.iter().copied().collect();

    // We'll attempt an O(n^2) approach:
    // For each pair (a, b), check if (2020 - (a+b)) is in the set.
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            let c = 2020 - (a + b);
            if set.contains(&c) {
                return a * b * c;
            }
        }
    }
    0
}

fn part_1(input: aoc::Input) -> impl ToString {
    // Parse each line in input to i64
    let numbers: Vec<i64> = input
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();

    let answer = solve_part_1(&numbers);
    answer.to_string()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let numbers: Vec<i64> = input
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();

    solve_part_2(&numbers).to_string()
}

// ------------------------
// Tests
// ------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        // Part 1 example from the prompt:
        // 1721 + 299 = 2020, product = 514579
        let example_input = vec![1721, 979, 366, 299, 675, 1456];
        let got = solve_part_1(&example_input);
        let want = 514579;
        assert_eq!(
            got, want,
            "Part 1 example failed: expected {want}, got {got}"
        );
    }

    #[test]
    fn test_part_2_example() {
        // Part 2 example from the prompt:
        // 979 + 366 + 675 = 2020, product = 241861950
        let example_input = vec![1721, 979, 366, 299, 675, 1456];
        let got = solve_part_2(&example_input);
        let want = 241861950;
        assert_eq!(
            got, want,
            "Part 2 example failed: expected {want}, got {got}"
        );
    }
}
