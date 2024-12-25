aoc::parts!(1, 2);

fn count_trees(grid: &[Vec<u8>], right: usize, down: usize) -> usize {
    let mut tree_count = 0;
    let mut x = 0;
    let width = grid[0].len();

    for y in (0..grid.len()).step_by(down) {
        if grid[y][x % width] == b'#' {
            tree_count += 1;
        }
        x += right;
    }

    tree_count
}

fn part_1(input: aoc::Input) -> impl ToString {
    let grid: Vec<Vec<u8>> = input
        .raw()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    count_trees(&grid, 3, 1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let grid: Vec<Vec<u8>> = input
        .raw()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes
        .iter()
        .map(|&(right, down)| count_trees(&grid, right, down))
        .product();

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_trees_example() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        assert_eq!(count_trees(&grid, 3, 1), 7);
    }

    #[test]
    fn test_count_trees_different_slope() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        assert_eq!(count_trees(&grid, 1, 1), 2);
    }

    #[test]
    fn test_part_2_example() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let product: usize = slopes
            .iter()
            .map(|&(right, down)| count_trees(&grid, right, down))
            .product();
        assert_eq!(product, 336);
    }
}
