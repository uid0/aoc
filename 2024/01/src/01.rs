use std::collections::HashMap;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    // Step 1: Parse input into lists.
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
    }

    // Step 2: Sort both lists
    left.sort();
    right.sort();

    // Step 3: Calculate the total distance
    let total_distance: i32 = left
        .iter()
        .zip(right.iter()) // Pairs up the elements from the sorted lists
        .map(|(l, r)| (l - r).abs()) // Calculates the absolute difference
        .sum(); // Sums up all the distances

    // Return the total distance as a string
    total_distance.to_string()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
    }

    // Count occurrences of each number in the right list
    let mut right_count: HashMap<i32, usize> = HashMap::new();

    for &num in &right {
        *right_count.entry(num).or_insert(0) += 1;
    }

    // Calculate the total similarity score
    let total_similarity_score: i32 = left
        .iter()
        .map(|&num| {
            let count = *right_count.get(&num).unwrap_or(&0); // Get count from the map, 0 if not found.
            num * count as i32
        })
        .sum(); // Sum up all the products

    // Return the total similarity score as a string
    total_similarity_score.to_string()
}
