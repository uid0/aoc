
aoc::parts!(1,2);

fn part_1(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let grid: Vec<Vec<char>> = raw_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let target = "XMAS";
    let target_len = target.len();
    let mut count = 0;

    let directions = [
        (0, 1),   // Horizontal right
        (1, 0),   // Vertical down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (0, -1),  // Horizontal left
        (-1, 0),  // Vertical up
        (-1, -1), // Diagonal up-left
        (-1, 1),  // Diagonal up-right
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;

                for i in 0..target_len {
                    let new_row = row as isize + i as isize * dr;
                    let new_col = col as isize + i as isize * dc;

                    if new_row < 0
                        || new_row >= rows as isize
                        || new_col < 0
                        || new_col >= cols as isize
                        || grid[new_row as usize][new_col as usize] != target.chars().nth(i).unwrap()
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_2(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let grid: Vec<Vec<char>> = raw_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Walk through every 'A' in the grid
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] != 'A' {
                continue;
            }

            let mut xmas_count = 0;

            // Top-left to bottom-right
            if row >= 1 && col >= 1 && row + 1 < rows && col + 1 < cols {
                if (grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S') || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M') {
                    xmas_count += 1;
                }
            }

            // Top-right to bottom-left
            if row >= 1 && col + 1 < cols && row + 1 < rows && col >= 1 {
                if (grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S') || (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M') {
                    xmas_count += 1;
                }
            }

            // Count valid X-MAS patterns for this 'A'
            count += xmas_count / 2; // Each pattern is counted twice, once per diagonal
        }
    }

    count
}