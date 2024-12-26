aoc::parts!(1, 2);

#[derive(Clone, PartialEq, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn parse_input(input: &aoc::Input) -> Vec<Vec<Seat>> {
    input
        .as_lines()
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    _ => panic!("Invalid input character"),
                })
                .collect()
        })
        .collect()
}

fn count_visible_occupied(grid: &Vec<Vec<Seat>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Directions: up, down, left, right, and diagonals
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (dr, dc) in directions.iter() {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;

        while r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
            match grid[r as usize][c as usize] {
                Seat::Occupied => {
                    count += 1;
                    break;
                }
                Seat::Empty => {
                    break;
                }
                Seat::Floor => {}
            }
            r += dr;
            c += dc;
        }
    }

    count
}

fn count_adjacent_occupied(grid: &Vec<Vec<Seat>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in (row.saturating_sub(1))..=(row + 1).min(rows - 1) {
        for j in (col.saturating_sub(1))..=(col + 1).min(cols - 1) {
            if (i != row || j != col) && grid[i][j] == Seat::Occupied {
                count += 1;
            }
        }
    }

    count
}

fn simulate_seating_part_2(grid: &mut Vec<Vec<Seat>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut changed = true;

    while changed {
        changed = false;
        let mut next_grid = grid.clone();

        for i in 0..rows {
            for j in 0..cols {
                let visible_occupied = count_visible_occupied(&grid, i, j);
                match grid[i][j] {
                    Seat::Empty => {
                        if visible_occupied == 0 {
                            next_grid[i][j] = Seat::Occupied;
                            changed = true;
                        }
                    }
                    Seat::Occupied => {
                        if visible_occupied >= 5 {
                            next_grid[i][j] = Seat::Empty;
                            changed = true;
                        }
                    }
                    Seat::Floor => {}
                }
            }
        }

        *grid = next_grid;
    }

    grid.iter()
        .map(|row| row.iter().filter(|&seat| *seat == Seat::Occupied).count())
        .sum()
}

fn simulate_seating_part_1(grid: &mut Vec<Vec<Seat>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut changed = true;

    while changed {
        changed = false;
        let mut next_grid = grid.clone();

        for i in 0..rows {
            for j in 0..cols {
                let occupied_adjacent = count_adjacent_occupied(&grid, i, j);
                match grid[i][j] {
                    Seat::Empty => {
                        if occupied_adjacent == 0 {
                            next_grid[i][j] = Seat::Occupied;
                            changed = true;
                        }
                    }
                    Seat::Occupied => {
                        if occupied_adjacent >= 4 {
                            next_grid[i][j] = Seat::Empty;
                            changed = true;
                        }
                    }
                    Seat::Floor => {}
                }
            }
        }

        *grid = next_grid;
    }

    grid.iter()
        .map(|row| row.iter().filter(|&seat| *seat == Seat::Occupied).count())
        .sum()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut grid = parse_input(&input);
    let occupied_seats = simulate_seating_part_1(&mut grid);
    occupied_seats
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut grid = parse_input(&input);
    let occupied_seats = simulate_seating_part_2(&mut grid);
    occupied_seats
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_vec(input_str: &str) -> Vec<Vec<Seat>> {
        input_str
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Seat::Floor,
                        'L' => Seat::Empty,
                        '#' => Seat::Occupied,
                        _ => panic!("Invalid input character"),
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_count_adjacent_occupied() {
        let grid = str_to_vec(
            "#.##.##.##\n\
             #######.##\n\
             #.#.#..#..\n\
             ####.##.##\n\
             #.##.##.##\n\
             #.#####.##\n\
             ..#.#.....\n\
             ##########\n\
             #.######.#\n\
             #.#####.##",
        );
        assert_eq!(count_adjacent_occupied(&grid, 0, 0), 2);
        assert_eq!(count_adjacent_occupied(&grid, 1, 1), 6);
        assert_eq!(count_adjacent_occupied(&grid, 9, 9), 2);
        assert_eq!(count_adjacent_occupied(&grid, 6, 6), 5);
    }

    #[test]
    fn test_simulate_seating_part_1() {
        let mut grid = str_to_vec(
            "L.LL.LL.LL\n\
             LLLLLLL.LL\n\
             L.L.L..L..\n\
             LLLL.LL.LL\n\
             L.LL.LL.LL\n\
             L.LLLLL.LL\n\
             ..L.L.....\n\
             LLLLLLLLLL\n\
             L.LLLLLL.L\n\
             L.LLLLL.LL",
        );
        let occupied_seats = simulate_seating_part_1(&mut grid);
        assert_eq!(occupied_seats, 37);
    }

    #[test]
    fn test_count_visible_occupied() {
        let grid = str_to_vec(
            ".......#.\n\
             ...#.....\n\
             .#.......\n\
             .........\n\
             ..#L....#\n\
             ....#....\n\
             .........\n\
             #........\n\
             ...#.....",
        );
        assert_eq!(count_visible_occupied(&grid, 4, 3), 8);

        let grid = str_to_vec(
            ".............\n\
             .L.L.#.#.#.#.\n\
             .............",
        );
        assert_eq!(count_visible_occupied(&grid, 1, 1), 0);

        let grid = str_to_vec(
            ".##.##.\n\
             #.#.#.#\n\
             ##...##\n\
             ...L...\n\
             ##...##\n\
             #.#.#.#\n\
             .##.##.",
        );
        assert_eq!(count_visible_occupied(&grid, 3, 3), 0);
    }

    #[test]
    fn test_simulate_seating_part_2() {
        let mut grid = str_to_vec(
            "L.LL.LL.LL\n\
             LLLLLLL.LL\n\
             L.L.L..L..\n\
             LLLL.LL.LL\n\
             L.LL.LL.LL\n\
             L.LLLLL.LL\n\
             ..L.L.....\n\
             LLLLLLLLLL\n\
             L.LLLLLL.L\n\
             L.LLLLL.LL",
        );
        let occupied_seats = simulate_seating_part_2(&mut grid);
        assert_eq!(occupied_seats, 26);
    }
}
