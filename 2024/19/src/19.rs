aoc::parts!(1, 2);

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part_1(input: aoc::Input) -> impl ToString {
    let (patterns, designs) = parse_input(&input);
    count_possible_designs(&patterns, &designs)
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let (patterns, designs) = parse_input(&input);
    count_total_ways(&patterns, &designs)
}

fn parse_input(input: &aoc::Input) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    // Find blank line separating patterns and designs
    let blank_line_pos = lines
        .iter()
        .position(|l| l.is_empty())
        .expect("Input should have a blank line after patterns line");

    let pattern_line = lines[0];
    let patterns = pattern_line
        .split(',')
        .map(|p| p.trim().to_string())
        .collect::<Vec<_>>();

    let designs = lines[(blank_line_pos + 1)..]
        .iter()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();

    (patterns, designs)
}

fn count_possible_designs(patterns: &[String], designs: &[String]) -> usize {
    let pattern_set: HashSet<&str> = HashSet::from_iter(patterns.iter().map(|p| p.as_str()));

    designs
        .iter()
        .filter(|d| can_form_design(d, &pattern_set))
        .count()
}

fn can_form_design(design: &str, patterns: &HashSet<&str>) -> bool {
    // DP to check possibility: same logic as part 1
    let n = design.len();
    let chars = design.as_bytes();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 0..n {
        if !dp[i] {
            continue;
        }
        for pat in patterns {
            let plen = pat.len();
            if i + plen <= n && &chars[i..i + plen] == pat.as_bytes() {
                dp[i + plen] = true;
            }
        }
    }

    dp[n]
}

fn count_total_ways(patterns: &[String], designs: &[String]) -> u128 {
    let pattern_set: Vec<&str> = patterns.iter().map(|p| p.as_str()).collect();

    let mut total = 0u128;
    for d in designs {
        total += count_ways(d, &pattern_set);
    }

    total
}

fn count_ways(design: &str, patterns: &[&str]) -> u128 {
    let n = design.len();
    let chars = design.as_bytes();
    let mut dp = vec![0u128; n + 1];
    dp[n] = 1; // base case: one way to complete from the end

    for i in (0..n).rev() {
        for pat in patterns {
            let plen = pat.len();
            if i + plen <= n && &chars[i..i + plen] == pat.as_bytes() {
                dp[i] = dp[i].saturating_add(dp[i + plen]);
            }
        }
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        // Example from the prompt (part 1):
        // Patterns: r, wr, b, g, bwu, rb, gb, br
        // Possible designs: brwrr, bggr, gbbr, rrbgbr, bwurrg, brgr
        // Impossible designs: ubwu, bbrgwb
        // Count: 6
        let patterns = vec![
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ];
        let designs = vec![
            "brwrr".to_string(),
            "bggr".to_string(),
            "gbbr".to_string(),
            "rrbgbr".to_string(),
            "ubwu".to_string(),
            "bwurrg".to_string(),
            "brgr".to_string(),
            "bbrgwb".to_string(),
        ];

        let result = count_possible_designs(&patterns, &designs);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_part2() {
        // From the prompt (part 2):
        // The sums of ways:
        // brwrr -> 2 ways
        // bggr -> 1 way
        // gbbr -> 4 ways
        // rrbgbr -> 6 ways
        // ubwu -> 0 ways
        // bwurrg -> 1 way
        // brgr -> 2 ways
        // bbrgwb -> 0 ways
        //
        // total = 2 + 1 + 4 + 6 + 0 + 1 + 2 + 0 = 16
        let patterns = vec![
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ];
        let designs = vec![
            "brwrr".to_string(),
            "bggr".to_string(),
            "gbbr".to_string(),
            "rrbgbr".to_string(),
            "ubwu".to_string(),
            "bwurrg".to_string(),
            "brgr".to_string(),
            "bbrgwb".to_string(),
        ];

        let result = count_total_ways(&patterns, &designs);
        assert_eq!(result, 16);
    }
}
