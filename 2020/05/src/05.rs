aoc::parts!(1, 2);

use std::collections::HashSet;

fn calculate_seat_id(boarding_pass: &str) -> u32 {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut col_min = 0;
    let mut col_max = 7;

    for c in boarding_pass.chars() {
        match c {
            'F' => row_max = row_min + (row_max - row_min) / 2,
            'B' => row_min = row_min + (row_max - row_min) / 2 + 1,
            'L' => col_max = col_min + (col_max - col_min) / 2,
            'R' => col_min = col_min + (col_max - col_min) / 2 + 1,
            _ => panic!("Invalid character in boarding pass"),
        }
    }

    row_min * 8 + col_min
}

fn part_1(input: aoc::Input) -> impl ToString {
    let highest_seat_id = input
        .raw()
        .lines()
        .map(calculate_seat_id)
        .max()
        .unwrap_or(0);

    highest_seat_id
}

fn part_2(input: aoc::Input) -> impl ToString {
    let seat_ids: HashSet<u32> = input.raw().lines().map(calculate_seat_id).collect();

    let my_seat_id = (1..127 * 8 + 7)
        .find(|&id| {
            !seat_ids.contains(&id) && seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1))
        })
        .unwrap_or(0);

    my_seat_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_seat_id_example_1() {
        assert_eq!(calculate_seat_id("FBFBBFFRLR"), 357);
    }

    #[test]
    fn test_calculate_seat_id_example_2() {
        assert_eq!(calculate_seat_id("BFFFBBFRRR"), 567);
    }

    #[test]
    fn test_calculate_seat_id_example_3() {
        assert_eq!(calculate_seat_id("FFFBBBFRRR"), 119);
    }

    #[test]
    fn test_calculate_seat_id_example_4() {
        assert_eq!(calculate_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_part_1_example() {
        let input = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        let boarding_passes: Vec<&str> = input.lines().collect();
        let highest_id: u32 = boarding_passes
            .iter()
            .map(|p| calculate_seat_id(p))
            .max()
            .unwrap();
        assert_eq!(highest_id, 820);
    }
}
