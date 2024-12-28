aoc::parts!(1, 2);

use std::collections::HashSet;

type Point3D = (i32, i32, i32);
type Point4D = (i32, i32, i32, i32);

fn neighbors_3d(p: &Point3D) -> Vec<Point3D> {
    let mut result = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                result.push((p.0 + x, p.1 + y, p.2 + z));
            }
        }
    }
    result
}

fn count_active_neighbors_3d(p: &Point3D, active_cubes: &HashSet<Point3D>) -> usize {
    neighbors_3d(p)
        .iter()
        .filter(|n| active_cubes.contains(n))
        .count()
}

fn solve_3d(initial_state: &str) -> usize {
    let mut active_cubes: HashSet<Point3D> = HashSet::new();
    for (y, line) in initial_state.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((x as i32, y as i32, 0));
            }
        }
    }

    for _cycle in 0..6 {
        let mut next_active_cubes: HashSet<Point3D> = HashSet::new();
        let mut to_check: HashSet<Point3D> = HashSet::new();

        for p in &active_cubes {
            to_check.insert(*p);
            for n in neighbors_3d(p) {
                to_check.insert(n);
            }
        }

        for p in to_check {
            let active_neighbors = count_active_neighbors_3d(&p, &active_cubes);
            if active_cubes.contains(&p) {
                if active_neighbors == 2 || active_neighbors == 3 {
                    next_active_cubes.insert(p);
                }
            } else {
                if active_neighbors == 3 {
                    next_active_cubes.insert(p);
                }
            }
        }
        active_cubes = next_active_cubes;
    }

    active_cubes.len()
}

fn neighbors_4d(p: &Point4D) -> Vec<Point4D> {
    let mut result = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    result.push((p.0 + x, p.1 + y, p.2 + z, p.3 + w));
                }
            }
        }
    }
    result
}

fn count_active_neighbors_4d(p: &Point4D, active_cubes: &HashSet<Point4D>) -> usize {
    neighbors_4d(p)
        .iter()
        .filter(|n| active_cubes.contains(n))
        .count()
}

fn solve_4d(initial_state: &str) -> usize {
    let mut active_cubes: HashSet<Point4D> = HashSet::new();
    for (y, line) in initial_state.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    for _cycle in 0..6 {
        let mut next_active_cubes: HashSet<Point4D> = HashSet::new();
        let mut to_check: HashSet<Point4D> = HashSet::new();

        for p in &active_cubes {
            to_check.insert(*p);
            for n in neighbors_4d(p) {
                to_check.insert(n);
            }
        }

        for p in to_check {
            let active_neighbors = count_active_neighbors_4d(&p, &active_cubes);
            if active_cubes.contains(&p) {
                if active_neighbors == 2 || active_neighbors == 3 {
                    next_active_cubes.insert(p);
                }
            } else {
                if active_neighbors == 3 {
                    next_active_cubes.insert(p);
                }
            }
        }
        active_cubes = next_active_cubes;
    }

    active_cubes.len()
}

fn part_1(input: aoc::Input) -> impl ToString {
    solve_3d(input.raw())
}

fn part_2(input: aoc::Input) -> impl ToString {
    solve_4d(input.raw())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = ".#.\n..#\n###";
        assert_eq!(solve_3d(input), 112);
    }

    #[test]
    fn test_part_2() {
        let input = ".#.\n..#\n###";
        assert_eq!(solve_4d(input), 848);
    }
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
