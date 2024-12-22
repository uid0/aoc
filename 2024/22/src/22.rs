aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let raw_data = input.raw();

    // Parse input: each line is an initial secret number for a buyer
    let initial_secrets: Vec<u64> = raw_data.lines().map(|line| line.parse().unwrap()).collect();

    // Calculate the 2000th secret number for each buyer and sum them
    let sum_2000th = initial_secrets
        .iter()
        .map(|&secret| simulate_secret(secret, 2000))
        .sum::<u64>();

    sum_2000th
}

fn part_2(input: aoc::Input) -> impl ToString {
    let raw_data = input.raw();

    // Parse input: each line is an initial secret number for a buyer
    let initial_secrets: Vec<u64> = raw_data.lines().map(|line| line.parse().unwrap()).collect();

    // Generate all possible sequences of four price changes (-9 to 9 inclusive)
    let mut max_bananas = 0;

    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let sequence = [a, b, c, d];
                    let total_bananas: u64 = initial_secrets
                        .iter()
                        .map(|&secret| {
                            calculate_bananas_for_sequence_optimized(secret, &sequence, 2000)
                        })
                        .sum();

                    if total_bananas > max_bananas {
                        max_bananas = total_bananas;
                    }
                }
            }
        }
    }

    max_bananas
}

fn simulate_secret(mut secret: u64, steps: usize) -> u64 {
    for _ in 0..steps {
        // Step 1: Multiply by 64 and mix
        secret ^= secret.wrapping_mul(64);
        secret %= 16777216; // Prune

        // Step 2: Divide by 32 and mix
        secret ^= secret / 32;
        secret %= 16777216; // Prune

        // Step 3: Multiply by 2048 and mix
        secret ^= secret.wrapping_mul(2048);
        secret %= 16777216; // Prune
    }

    secret
}

fn calculate_bananas_for_sequence_optimized(secret: u64, sequence: &[i32; 4], steps: usize) -> u64 {
    let mut current_secret = secret;
    let mut last_price = (current_secret % 10) as i32;
    let mut changes = [0; 4];
    let mut match_idx = 0;

    for _ in 0..steps {
        // Generate next price
        current_secret = simulate_secret(current_secret, 1);
        let price = (current_secret % 10) as i32;

        // Calculate the price change
        let change = price - last_price;
        last_price = price;

        // Update the rolling changes array
        changes[match_idx] = change;
        match_idx = (match_idx + 1) % 4;

        // Check if the current sequence matches
        if changes
            .iter()
            .cycle()
            .skip(match_idx)
            .take(4)
            .eq(sequence.iter())
        {
            return price as u64; // Return the price if sequence matches
        }
    }

    0 // No match found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "1\n10\n100\n2024";
        let initial_secrets: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

        let results: Vec<u64> = initial_secrets
            .iter()
            .map(|&secret| simulate_secret(secret, 2000))
            .collect();

        assert_eq!(results, vec![8685429, 4700978, 15273692, 8667524]);
        assert_eq!(results.iter().sum::<u64>(), 37327623);
    }

    #[test]
    fn test_part_2_example() {
        let input = "1\n2\n3\n2024";
        let initial_secrets: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

        let sequence = [-2, 1, -1, 3];
        let bananas: u64 = initial_secrets
            .iter()
            .map(|&secret| calculate_bananas_for_sequence_optimized(secret, &sequence, 2000))
            .sum();

        assert_eq!(bananas, 23);
    }
}
