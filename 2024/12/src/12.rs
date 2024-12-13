use std::collections::{HashSet, VecDeque};

aoc::parts!(1, 2);
fn part_1(input: aoc::Input) -> impl ToString {
    let map = input
        .raw()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if !visited.contains(&(x, y)) {
                let (area, perimeter) =
                    calculate_area_and_perimeter(&map, x, y, cell, &mut visited);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn part_2(input: aoc::Input) -> impl ToString {
    let map = input
        .raw()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if !visited.contains(&(x, y)) {
                let (area, sides) = calculate_area_and_sides(&map, x, y, cell, &mut visited);
                total_price += area * sides;
            }
        }
    }

    total_price
}

fn calculate_area_and_perimeter(
    map: &[Vec<char>],
    start_x: usize,
    start_y: usize,
    region_char: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;

    queue.push_back((start_x, start_y));
    visited.insert((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        area += 1;
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx >= map[0].len() as isize || ny >= map.len() as isize {
                perimeter += 1;
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if map[ny][nx] != region_char {
                perimeter += 1;
            } else if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push_back((nx, ny));
            }
        }
    }

    (area, perimeter)
}

fn calculate_area_and_sides(
    map: &[Vec<char>],
    start_x: usize,
    start_y: usize,
    region_char: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut area = 0;
    let mut sides = 0;

    queue.push_back((start_x, start_y));
    visited.insert((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        area += 1;
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx >= map[0].len() as isize || ny >= map.len() as isize {
                sides += 1;
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if map[ny][nx] != region_char {
                sides += 1;
            }
        }
    }

    (area, sides)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_and_perimeter() {
        let map = vec![
            vec!['A', 'A', 'A'],
            vec!['A', 'B', 'A'],
            vec!['A', 'A', 'A'],
        ];
        let mut visited = HashSet::new();
        let (area, perimeter) = calculate_area_and_perimeter(&map, 0, 0, 'A', &mut visited);
        assert_eq!(area, 8);
        assert_eq!(perimeter, 12);
    }

    #[test]
    fn test_area_and_sides() {
        let map = vec![
            vec!['A', 'A', 'A'],
            vec!['A', 'B', 'A'],
            vec!['A', 'A', 'A'],
        ];
        let mut visited = HashSet::new();
        let (area, sides) = calculate_area_and_sides(&map, 0, 0, 'A', &mut visited);
        assert_eq!(area, 8);
        assert_eq!(sides, 12);
    }

    #[test]
    fn test_simple_perimeter() {
        let map = vec![vec!['A', 'A'], vec!['A', 'A']];
        let mut visited = HashSet::new();
        let (area, perimeter) = calculate_area_and_perimeter(&map, 0, 0, 'A', &mut visited);
        assert_eq!(area, 4);
        assert_eq!(perimeter, 8);
    }

    #[test]
    fn test_simple_sides() {
        let map = vec![vec!['A', 'A'], vec!['A', 'A']];
        let mut visited = HashSet::new();
        let (area, sides) = calculate_area_and_sides(&map, 0, 0, 'A', &mut visited);
        assert_eq!(area, 4);
        assert_eq!(sides, 8);
    }
}
