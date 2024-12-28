use std::collections::HashMap;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let line = input.as_lines()[0];
    let starting_numbers: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
    solve(starting_numbers, 2020)
}

fn solve(starting_numbers: Vec<u32>, nth_number: u32) -> u32 {
    let mut spoken_numbers: Vec<i32> = vec![-1; nth_number as usize];
    let mut last_number_spoken: u32 = 0;

    for (turn, &number) in starting_numbers.iter().enumerate() {
        spoken_numbers[number as usize] = turn as i32 + 1;
        last_number_spoken = number;
    }

    for turn in starting_numbers.len() as u32..nth_number {
        let next_number = match spoken_numbers[last_number_spoken as usize] {
            -1 => 0,
            last_turn_spoken => turn - last_turn_spoken as u32,
        };
        spoken_numbers[last_number_spoken as usize] = turn as i32;
        last_number_spoken = next_number;
    }

    last_number_spoken
}

fn part_2(input: aoc::Input) -> impl ToString {
    let line = input.as_lines()[0];
    let starting_numbers: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
    solve(starting_numbers, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(solve(vec![0, 3, 6], 2020), 436);
        assert_eq!(solve(vec![1, 3, 2], 2020), 1);
        assert_eq!(solve(vec![2, 1, 3], 2020), 10);
        assert_eq!(solve(vec![1, 2, 3], 2020), 27);
        assert_eq!(solve(vec![2, 3, 1], 2020), 78);
        assert_eq!(solve(vec![3, 2, 1], 2020), 438);
        assert_eq!(solve(vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_examples_part_2() {
        assert_eq!(solve(vec![0, 3, 6], 30000000), 175594);
        assert_eq!(solve(vec![1, 3, 2], 30000000), 2578);
        assert_eq!(solve(vec![2, 1, 3], 30000000), 3544142);
        assert_eq!(solve(vec![1, 2, 3], 30000000), 261214);
        assert_eq!(solve(vec![2, 3, 1], 30000000), 6895259);
        assert_eq!(solve(vec![3, 2, 1], 30000000), 18);
        assert_eq!(solve(vec![3, 1, 2], 30000000), 362);
    }
}
