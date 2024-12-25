aoc::parts!(1, 2);

use std::collections::VecDeque;

fn is_valid_xmas_number(preamble: &VecDeque<i64>, number: i64) -> bool {
    for i in 0..preamble.len() {
        for j in i + 1..preamble.len() {
            if preamble[i] + preamble[j] == number {
                return true;
            }
        }
    }
    false
}

fn find_first_invalid_number(numbers: &[i64], preamble_length: usize) -> Option<i64> {
    let mut preamble = VecDeque::with_capacity(preamble_length);

    // Initialize the preamble
    for &number in numbers.iter().take(preamble_length) {
        preamble.push_back(number);
    }

    // Check subsequent numbers
    for &number in numbers.iter().skip(preamble_length) {
        if !is_valid_xmas_number(&preamble, number) {
            return Some(number);
        }
        preamble.pop_front();
        preamble.push_back(number);
    }

    None // No invalid number found
}

fn find_contiguous_sum(numbers: &[i64], target: i64) -> Option<&[i64]> {
    for i in 0..numbers.len() {
        let mut sum = 0;
        for j in i..numbers.len() {
            sum += numbers[j];
            if sum == target {
                return Some(&numbers[i..=j]);
            } else if sum > target {
                break; // No need to continue if the sum exceeds the target
            }
        }
    }
    None // No contiguous sum found
}

fn part_1(input: aoc::Input) -> impl ToString {
    let numbers: Vec<i64> = input
        .raw()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let preamble_length = 25;

    find_first_invalid_number(&numbers, preamble_length).unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let numbers: Vec<i64> = input
        .raw()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let preamble_length = 25;

    let invalid_number = find_first_invalid_number(&numbers, preamble_length).unwrap();

    let contiguous_range = find_contiguous_sum(&numbers, invalid_number).unwrap();
    let min = contiguous_range.iter().min().unwrap();
    let max = contiguous_range.iter().max().unwrap();

    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_xmas_number() {
        let mut preamble = VecDeque::from(vec![1, 2, 3, 4, 5]);
        assert!(is_valid_xmas_number(&preamble, 6));
        assert!(!is_valid_xmas_number(&preamble, 10));
        preamble.pop_front();
        preamble.push_back(6);
        assert!(is_valid_xmas_number(&preamble, 10));
        assert!(!is_valid_xmas_number(&preamble, 12));
    }

    #[test]
    fn test_find_first_invalid_number_example() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let preamble_length = 5;
        assert_eq!(
            find_first_invalid_number(&numbers, preamble_length),
            Some(127)
        );
    }

    #[test]
    fn test_part_1_example() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        let numbers: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
        let preamble_length = 5;
        assert_eq!(
            find_first_invalid_number(&numbers, preamble_length),
            Some(127)
        );
    }

    #[test]
    fn test_find_contiguous_sum_example() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let target = 127;
        let expected_range = &numbers[2..=5]; // [15, 25, 47, 40]
        assert_eq!(find_contiguous_sum(&numbers, target), Some(expected_range));
    }

    #[test]
    fn test_part_2_example() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        let numbers: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
        let preamble_length = 5;
        let invalid_number = find_first_invalid_number(&numbers, preamble_length).unwrap();
        let contiguous_range = find_contiguous_sum(&numbers, invalid_number).unwrap();
        let min = contiguous_range.iter().min().unwrap();
        let max = contiguous_range.iter().max().unwrap();
        assert_eq!(min + max, 62);
    }
}
