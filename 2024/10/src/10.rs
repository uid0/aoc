use std::collections::{HashSet, VecDeque};

aoc::parts!(1, 2);

fn calculate_trailhead_score(grid: &[Vec<u8>], start_r: usize, start_c: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut reachable_nines = HashSet::new();

    visited.insert((start_r, start_c));
    queue.push_back((start_r, start_c, 0)); // (row, col, height)

    while let Some((r, c, height)) = queue.pop_front() {
        if grid[r][c] == 9 {
            reachable_nines.insert((r, c));
        }

        for &(dr, dc) in &directions {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr < 0 || nc < 0 || nr >= grid.len() as isize || nc >= grid[0].len() as isize {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);

            if !visited.contains(&(nr, nc)) && grid[nr][nc] == height + 1 {
                visited.insert((nr, nc));
                queue.push_back((nr, nc, grid[nr][nc]));
            }
        }
    }

    reachable_nines.len()
}

fn calculate_trailhead_rating(grid: &[Vec<u8>], start_r: usize, start_c: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut distinct_trails = HashSet::new();

    fn dfs(
        grid: &[Vec<u8>],
        r: usize,
        c: usize,
        height: u8,
        path: &mut Vec<(usize, usize)>,
        distinct_trails: &mut HashSet<Vec<(usize, usize)>>,
        directions: &[(isize, isize)],
    ) {
        path.push((r, c));
        if grid[r][c] == 9 {
            distinct_trails.insert(path.clone());
            path.pop();
            return;
        }

        for &(dr, dc) in directions {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr < 0 || nc < 0 || nr >= grid.len() as isize || nc >= grid[0].len() as isize {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == height + 1 && !path.contains(&(nr, nc)) {
                dfs(
                    grid,
                    nr,
                    nc,
                    grid[nr][nc],
                    path,
                    distinct_trails,
                    directions,
                );
            }
        }

        path.pop();
    }

    let mut path = Vec::new();
    dfs(
        grid,
        start_r,
        start_c,
        grid[start_r][start_c],
        &mut path,
        &mut distinct_trails,
        &directions,
    );

    distinct_trails.len()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let grid: Vec<Vec<u8>> = raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut total_score = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 0 {
                total_score += calculate_trailhead_score(&grid, r, c);
            }
        }
    }

    total_score
}

fn part_2(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let grid: Vec<Vec<u8>> = raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut total_rating = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 0 {
                total_rating += calculate_trailhead_rating(&grid, r, c);
            }
        }
    }

    total_rating
}

mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "10..9..\n2...8..\n3...7..\n4567654\n...8..3\n...9..2\n.....01";
        let result = part_1(input);
        assert_eq!(result.to_string(), "3"); // Expected score
    }

    #[test]
    fn test_part_2_small() {
        let input = ".....0.\n..4321.\n..5..2.\n..6543.\n..7..4.\n..8765.\n..9....";
        let result = part_2(input);
        assert_eq!(result.to_string(), "3"); // Expected rating
    }

    #[test]
    fn test_part_2_larger() {
        let input_data =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let result = part_2(input);
        assert_eq!(result.to_string(), "81"); // Expected rating
    }
}
