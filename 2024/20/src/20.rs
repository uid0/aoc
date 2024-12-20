aoc::parts!(1, 2);

use std::collections::{HashMap, HashSet, VecDeque};

fn part_1(input: aoc::Input) -> impl ToString {
    let (grid, start, end) = parse_input(input.raw());
    let shortest_path = bfs(&grid, start, end).unwrap();
    println!("Shortest path: {}", shortest_path);
    let cheats = find_cheats(&grid, start, end, shortest_path, 2);
    println!("Cheats: {:?}", cheats);

    let mut count = 0;
    for (saved_time, num_cheats) in cheats.iter() {
        if *saved_time >= 100 {
            count += num_cheats;
        }
    }
    count
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (grid, start, end) = parse_input(input.raw());
    let shortest_path = bfs(&grid, start, end).unwrap();
    let cheats = find_cheats(&grid, start, end, shortest_path, 20);

    let mut count = 0;
    for (saved_time, num_cheats) in cheats.iter() {
        if *saved_time >= 100 {
            count += num_cheats;
        }
    }
    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Point, Point) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = Point { x, y };
            } else if cell == 'E' {
                end = Point { x, y };
            }
        }
    }
    (grid, start, end)
}

fn bfs(grid: &Vec<Vec<char>>, start: Point, end: Point) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((current, distance)) = queue.pop_front() {
        if current == end {
            return Some(distance);
        }

        let neighbors = get_neighbors(grid, current, false);
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    None
}

fn get_neighbors(grid: &Vec<Vec<char>>, point: Point, can_pass_through_walls: bool) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    for (dx, dy) in directions.iter() {
        let new_x = point.x as isize + dx;
        let new_y = point.y as isize + dy;

        if new_x >= 0 && new_x < grid[0].len() as isize && new_y >= 0 && new_y < grid.len() as isize
        {
            let new_point = Point {
                x: new_x as usize,
                y: new_y as usize,
            };
            if can_pass_through_walls || grid[new_point.y][new_point.x] != '#' {
                neighbors.push(new_point);
            }
        }
    }

    neighbors
}

fn find_cheats(
    grid: &Vec<Vec<char>>,
    start: Point,
    end: Point,
    shortest_path: usize,
    max_cheat: usize,
) -> HashMap<usize, usize> {
    let mut cheats = HashMap::new();
    let mut visited_starts = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let start_point = Point { x, y };
            println!("Checking start point: {:?}", start_point); // Debug
            if grid[y][x] != '#' && !visited_starts.contains(&start_point) {
                if let Some(dist_to_start) = bfs(grid, start, start_point) {
                    visited_starts.insert(start_point);
                    println!("  Distance to start: {}", dist_to_start); // Debug
                    for cheat_len in 1..=max_cheat {
                        println!("    Checking cheat length: {}", cheat_len); // Debug
                        let mut queue = VecDeque::new();
                        queue.push_back((start_point, 0, cheat_len));
                        let mut visited = HashSet::new();
                        visited.insert((start_point, cheat_len));
                        while let Some((current, dist_from_start, remaining_cheat)) =
                            queue.pop_front()
                        {
                            println!(
                                "Current: {:?}, dist: {}, remaining: {}",
                                current, dist_from_start, remaining_cheat
                            ); // Debug
                            if remaining_cheat == 0 && grid[current.y][current.x] != '#' {
                                if let Some(dist_to_end) = bfs(grid, current, end) {
                                    println!("        Distance to end: {}", dist_to_end); // Debug
                                    let total_dist = dist_to_start + dist_to_end + cheat_len;
                                    let saved_time = shortest_path as i32 - total_dist as i32;
                                    if saved_time > 0 {
                                        println!("          Saved time: {}", saved_time); // Debug
                                        *cheats.entry(saved_time as usize).or_insert(0) += 1;
                                    }
                                }
                            } else if remaining_cheat > 0 {
                                let neighbors = get_neighbors(grid, current, true);
                                for neighbor in neighbors {
                                    let next_state = (neighbor, remaining_cheat - 1);
                                    if !visited.contains(&next_state) {
                                        visited.insert(next_state);
                                        queue.push_back((
                                            neighbor,
                                            dist_from_start + 1,
                                            remaining_cheat - 1,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    cheats
}
