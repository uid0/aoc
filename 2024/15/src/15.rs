use std::collections::{HashMap, HashSet};
use std::io::BufRead;

aoc::parts!(1, 2);

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    fn add(self, dx: i64, dy: i64) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    solve(input, false)
}

fn part_2(input: aoc::Input) -> impl ToString {
    solve(input, true)
}

/// This function solves the puzzle. If `part_2` is set to false, it solves part 1.
/// If `part_2` is set to true, it solves part 2. For now, we only know part 1's logic,
/// but we structure the code for part 2 as well.
fn solve(mut input: aoc::Input, part_2: bool) -> impl ToString {
    // Convert the entire input into a string, then split into lines ourselves.
    let input_str = input.raw();
    let lines: Vec<&str> = input_str.lines().collect();

    // Separate map lines from moves
    let map_width = lines[0].len();
    let mut map_lines = Vec::new();
    let mut i = 0;
    for line in &lines {
        if line.len() == map_width && line.starts_with('#') && line.ends_with('#') {
            map_lines.push(*line);
        } else {
            break;
        }
        i += 1;
    }

    let move_lines = &lines[i..];
    let moves_str: String = move_lines
        .iter()
        .flat_map(|l| l.chars())
        .filter(|c| !c.is_whitespace())
        .collect();
    let moves: Vec<char> = moves_str.chars().collect();

    let mut grid = HashMap::new();
    let rows = map_lines.len();
    let cols = map_lines[0].len();

    for (y, line) in map_lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as i64, y as i64), ch);
        }
    }

    let mut robot = Point::new(0, 0);
    'find_robot: for y in 0..rows {
        for x in 0..cols {
            if let Some(ch) = grid.get(&(x as i64, y as i64)) {
                if *ch == '@' {
                    robot = Point::new(x as i64, y as i64);
                    break 'find_robot;
                }
            }
        }
    }

    if part_2 {
        let mut new_grid = HashMap::new();
        for ((x, y), ch) in &grid {
            match *ch {
                '#' => {
                    new_grid.insert((2 * x, *y), '#');
                    new_grid.insert((2 * x + 1, *y), '#');
                }
                'O' => {
                    new_grid.insert((2 * x, *y), '[');
                    new_grid.insert((2 * x + 1, *y), ']');
                }
                '.' => {
                    new_grid.insert((2 * x, *y), '.');
                    new_grid.insert((2 * x + 1, *y), '.');
                }
                '@' => {
                    new_grid.insert((2 * x, *y), '@');
                    new_grid.insert((2 * x + 1, *y), '.');
                }
                _ => {
                    new_grid.insert((2 * x, *y), '.');
                    new_grid.insert((2 * x + 1, *y), '.');
                }
            }
        }
        grid = new_grid;

        'find_robot_2: for (pos, ch) in &grid {
            if *ch == '@' {
                robot = Point::new(pos.0, pos.1);
                break 'find_robot_2;
            }
        }
    }

    fn direction_offset(d: char) -> (i64, i64, bool) {
        match d {
            '^' => (0, -1, true),
            'v' => (0, 1, true),
            '<' => (-1, 0, false),
            '>' => (1, 0, false),
            _ => (0, 0, false),
        }
    }

    let get_char = |grid: &HashMap<(i64, i64), char>, x: i64, y: i64| -> char {
        *grid.get(&(x, y)).unwrap_or(&'.')
    };

    let set_char = |grid: &mut HashMap<(i64, i64), char>, x: i64, y: i64, c: char| {
        grid.insert((x, y), c);
    };

    let is_wall = |ch: char| ch == '#';
    let is_floor = |ch: char| ch == '.';
    let is_box_part1 = |ch: char| ch == 'O';
    let is_box_part2 = |ch: char| ch == '[' || ch == ']';

    for &m in &moves {
        let (dx, dy, up_down) = direction_offset(m);
        let nr = robot.x + dx;
        let nc = robot.y + dy;

        if part_2 {
            let next_ch = get_char(&grid, nr, nc);
            if up_down {
                // Vertical pushing logic for part 2
                let mut frontier = vec![(robot.x, robot.y)];
                let mut to_move: Vec<((i64, i64), char)> = Vec::new();
                let mut found_empty = false;

                'vertical_loop: loop {
                    let mut all_floor = true;
                    let mut any_wall = false;
                    let mut hit = Vec::new();

                    for &(x, y) in &frontier {
                        let x2 = x + dx;
                        let y2 = y + dy;
                        let ch = get_char(&grid, x2, y2);
                        if is_floor(ch) {
                            // floor
                        } else if is_wall(ch) {
                            any_wall = true;
                            break;
                        } else if ch == '[' {
                            all_floor = false;
                            hit.push((x2, y2, '['));
                            let ch2 = get_char(&grid, x2 + 1, y2);
                            if ch2 == ']' {
                                hit.push((x2 + 1, y2, ']'));
                            }
                        } else if ch == ']' {
                            all_floor = false;
                            hit.push((x2, y2, ']'));
                            let ch2 = get_char(&grid, x2 - 1, y2);
                            if ch2 == '[' {
                                hit.push((x2 - 1, y2, '['));
                            }
                        } else if ch == '@' {
                            // treat as floor
                        } else {
                            // treat unknown as floor
                        }
                    }

                    if any_wall {
                        break 'vertical_loop;
                    }

                    if all_floor && hit.is_empty() {
                        found_empty = true;
                        break 'vertical_loop;
                    }

                    if !hit.is_empty() {
                        // Add boxes
                        let mut uniq = HashSet::new();
                        for (x, y, ch) in hit {
                            uniq.insert((x, y, ch));
                        }
                        for (bx, by, bch) in uniq {
                            if !to_move.iter().any(|((xx, yy), _)| *xx == bx && *yy == by) {
                                to_move.push(((bx, by), bch));
                            }
                        }
                        frontier = to_move.iter().map(|&((xx, yy), _)| (xx, yy)).collect();
                    } else {
                        break 'vertical_loop;
                    }
                }

                if found_empty {
                    let new_robot_x = robot.x + dx;
                    let new_robot_y = robot.y + dy;
                    set_char(&mut grid, robot.x, robot.y, '.');
                    for &((bx, by), _) in &to_move {
                        set_char(&mut grid, bx, by, '.');
                    }
                    for &((bx, by), ch) in &to_move {
                        let nx = bx + dx;
                        let ny = by + dy;
                        set_char(&mut grid, nx, ny, ch);
                    }
                    set_char(&mut grid, new_robot_x, new_robot_y, '@');
                    robot.x = new_robot_x;
                    robot.y = new_robot_y;
                } else if found_empty && to_move.is_empty() {
                    // Just move robot if empty and no boxes
                    let new_robot_x = robot.x + dx;
                    let new_robot_y = robot.y + dy;
                    if is_floor(get_char(&grid, new_robot_x, new_robot_y)) {
                        set_char(&mut grid, robot.x, robot.y, '.');
                        set_char(&mut grid, new_robot_x, new_robot_y, '@');
                        robot.x = new_robot_x;
                        robot.y = new_robot_y;
                    }
                }
            } else {
                // Horizontal logic part_2
                if is_wall(next_ch) {
                    continue;
                } else if is_floor(next_ch) {
                    set_char(&mut grid, robot.x, robot.y, '.');
                    set_char(&mut grid, nr, nc, '@');
                    robot.x = nr;
                    robot.y = nc;
                } else if next_ch == '[' || next_ch == ']' {
                    // Push horizontally
                    let mut positions = Vec::new();
                    let mut xcur = nr;
                    let mut ycur = nc;
                    loop {
                        let ch = get_char(&grid, xcur, ycur);
                        if ch == '[' {
                            let ch2 = get_char(&grid, xcur + 1, ycur);
                            if ch2 == ']' {
                                positions.push((xcur, ycur, '['));
                                positions.push((xcur + 1, ycur, ']'));
                                xcur += 2 * dx;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    let final_x = xcur;
                    let final_y = ycur;
                    if !is_floor(get_char(&grid, final_x, final_y)) {
                        continue;
                    }
                    set_char(&mut grid, robot.x, robot.y, '.');
                    for &(bx, by, _) in &positions {
                        set_char(&mut grid, bx, by, '.');
                    }
                    for &(bx, by, ch) in &positions {
                        let nx = bx + dx;
                        let ny = by + dy;
                        set_char(&mut grid, nx, ny, ch);
                    }
                    set_char(&mut grid, nr, nc, '@');
                    robot.x = nr;
                    robot.y = nc;
                }
            }
        } else {
            // Part 1 logic
            let next_ch = get_char(&grid, robot.x + dx, robot.y + dy);
            if is_wall(next_ch) {
                continue;
            } else if is_floor(next_ch) {
                set_char(&mut grid, robot.x, robot.y, '.');
                set_char(&mut grid, robot.x + dx, robot.y + dy, '@');
                robot.x += dx;
                robot.y += dy;
            } else if is_box_part1(next_ch) {
                // Push chain part 1
                let mut box_positions = Vec::new();
                let mut rx = robot.x + dx;
                let mut ry = robot.y + dy;
                while is_box_part1(get_char(&grid, rx, ry)) {
                    box_positions.push((rx, ry));
                    rx += dx;
                    ry += dy;
                }
                if !is_floor(get_char(&grid, rx, ry)) {
                    continue;
                }

                set_char(&mut grid, robot.x, robot.y, '.');
                set_char(&mut grid, rx, ry, 'O');
                for i in (1..box_positions.len()).rev() {
                    let dst = box_positions[i];
                    let _src = box_positions[i - 1]; // no need to use _src, just omit it
                    set_char(&mut grid, dst.0, dst.1, 'O');
                }
                let first_box = box_positions[0];
                set_char(&mut grid, first_box.0, first_box.1, '@');
                robot.x = first_box.0;
                robot.y = first_box.1;
            }
        }
    }

    let mut sum = 0_i64;
    if !part_2 {
        for ((x, y), ch) in &grid {
            if *ch == 'O' {
                sum += 100 * (*y as i64) + (*x as i64);
            }
        }
    } else {
        for ((x, y), ch) in &grid {
            if *ch == '[' {
                sum += 100 * (*y as i64) + (*x as i64);
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small_example_part1() {
        let input_str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
<^^>>>vv<v>>v<<";
        let input = aoc::Input::raw(input_str);
        let result = part_1(input).to_string();
        assert_eq!(result, "2028");
    }
}
