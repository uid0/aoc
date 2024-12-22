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
    let mut best_sequence = vec![0; 4];
    let mut max_bananas = 0;

    for seq in (-9..=9).flat_map(|a| {
        (-9..=9)
            .flat_map(move |b| (-9..=9).flat_map(move |c| (-9..=9).map(move |d| vec![a, b, c, d])))
    }) {
        let total_bananas: u64 = initial_secrets
            .iter()
            .map(|&secret| calculate_bananas_for_sequence(secret, &seq, 2000))
            .sum();

        if total_bananas > max_bananas {
            max_bananas = total_bananas;
            best_sequence = seq.clone();
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

fn calculate_bananas_for_sequence(secret: u64, sequence: &[i32], steps: usize) -> u64 {
    let mut prices = vec![];
    let mut changes = vec![];
    let mut current_secret = secret;

    for _ in 0..steps {
        let price = (current_secret % 10) as i32;
        prices.push(price);

        if prices.len() > 1 {
            let change = price - prices[prices.len() - 2];
            changes.push(change);

            if changes.len() >= 4 {
                if &changes[changes.len() - 4..] == sequence {
                    return price as u64;
                }
            }
        }

        current_secret = simulate_secret(current_secret, 1);
    }

    0
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

        let sequence = vec![-2, 1, -1, 3];
        let bananas: u64 = initial_secrets
            .iter()
            .map(|&secret| calculate_bananas_for_sequence(secret, &sequence, 2000))
            .sum();

        assert_eq!(bananas, 23);
    }
}
