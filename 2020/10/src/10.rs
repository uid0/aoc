aoc::parts!(1, 2);
use std::collections::HashMap;

fn calculate_joltage_differences(adapters: &mut Vec<u32>) -> (u32, u32) {
    adapters.sort_unstable();
    let mut one_jolt_diffs = 0;
    let mut three_jolt_diffs = 0;
    let mut current_joltage = 0;

    for adapter in adapters {
        let diff = *adapter - current_joltage;
        match diff {
            1 => one_jolt_diffs += 1,
            3 => three_jolt_diffs += 1,
            _ => {}
        }
        current_joltage = *adapter;
    }

    // Add the difference to the device's built-in adapter
    three_jolt_diffs += 1;

    (one_jolt_diffs, three_jolt_diffs)
}

fn count_arrangements(adapters: &Vec<u32>) -> u64 {
    let mut sorted_adapters = adapters.clone();
    sorted_adapters.sort_unstable();
    sorted_adapters.insert(0, 0); // Add the charging outlet (0 jolts)
    let max_joltage = sorted_adapters.last().unwrap() + 3;
    sorted_adapters.push(max_joltage); // Add the device

    let mut paths_to_reach: HashMap<u32, u64> = HashMap::new();
    paths_to_reach.insert(0, 1); // There's one way to reach the starting joltage (0)

    for i in 0..sorted_adapters.len() {
        let adapter = sorted_adapters[i];
        let current_paths = *paths_to_reach.get(&adapter).unwrap_or(&0);

        for j in (i + 1)..sorted_adapters.len() {
            let next_adapter = sorted_adapters[j];
            if next_adapter - adapter <= 3 {
                *paths_to_reach.entry(next_adapter).or_insert(0) += current_paths;
            } else {
                break; // since it's sorted, no need to continue inner loop
            }
        }
    }

    *paths_to_reach.get(&max_joltage).unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let adapters: Vec<u32> = input
        .as_lines()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    count_arrangements(&adapters)
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut adapters: Vec<u32> = input
        .as_lines()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let (one_jolt, three_jolt) = calculate_joltage_differences(&mut adapters);
    one_jolt * three_jolt
}

// Helper function for testing
#[cfg(test)]
fn calculate_joltage_product(mut adapters: Vec<u32>) -> u32 {
    let (one_jolt, three_jolt) = calculate_joltage_differences(&mut adapters);
    one_jolt * three_jolt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_joltage_differences_small() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let (one_jolt, three_jolt) = calculate_joltage_differences(&mut adapters.clone());
        assert_eq!(one_jolt, 7);
        assert_eq!(three_jolt, 5);
    }

    #[test]
    fn test_calculate_joltage_differences_large() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let (one_jolt, three_jolt) = calculate_joltage_differences(&mut adapters.clone());
        assert_eq!(one_jolt, 22);
        assert_eq!(three_jolt, 10);
    }

    #[test]
    fn test_part_1_small() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = calculate_joltage_product(adapters);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part_1_large() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result = calculate_joltage_product(adapters);
        assert_eq!(result, 220);
    }

    #[test]
    fn test_count_arrangements_small() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = count_arrangements(&adapters);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_count_arrangements_large() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result = count_arrangements(&adapters);
        assert_eq!(result, 19208);
    }
}
