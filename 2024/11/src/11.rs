use num_bigint::BigUint;
use num_traits::FromPrimitive;
use rayon::prelude::*;
use std::str::FromStr;

aoc::parts!(1, 2);

fn remove_leading_zeros(s: &str) -> String {
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

fn blink(stones: Vec<String>) -> Vec<String> {
    let two_thousand_twenty_four = BigUint::from_u32(2024).unwrap();

    // Process each stone in parallel:
    stones
        .into_par_iter()
        .flat_map(|stone| {
            if stone == "0" {
                // Rule 1: 0 -> 1
                vec!["1".to_string()]
            } else if stone.len() % 2 == 0 {
                // Rule 2: even length -> split into two stones
                let mid = stone.len() / 2;
                let left_part = &stone[..mid];
                let right_part = &stone[mid..];
                vec![
                    remove_leading_zeros(left_part),
                    remove_leading_zeros(right_part),
                ]
            } else {
                // Rule 3: multiply by 2024
                let num = BigUint::from_str(&stone).expect("valid number");
                let multiplied = &num * &two_thousand_twenty_four;
                vec![multiplied.to_string()]
            }
        })
        .collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut stones: Vec<String> = input
        .raw()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.len()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut stones: Vec<String> = input
        .raw()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    for _ in 0..75 {
        stones = blink(stones);
    }

    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = application::Input::new("125 17");
        let result = part_1(input).to_string();
        assert_eq!(result, "55312");
    }

    #[test]
    fn test_part_2_trivial() {
        let input = application::Input::new("0");
        let result = part_2(input).to_string().parse::<usize>().unwrap();
        assert!(result > 0);
    }
}
