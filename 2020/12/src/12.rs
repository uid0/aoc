aoc::parts!(1, 2);

#[derive(Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32,
}

fn parse_input(input: &aoc::Input) -> Vec<Instruction> {
    input
        .as_lines()
        .iter()
        .map(|line| {
            let action = match &line[0..1] {
                "N" => Action::North,
                "S" => Action::South,
                "E" => Action::East,
                "W" => Action::West,
                "L" => Action::Left,
                "R" => Action::Right,
                "F" => Action::Forward,
                _ => panic!("Invalid action"),
            };
            let value = line[1..].parse().unwrap();
            Instruction { action, value }
        })
        .collect()
}

fn calculate_manhattan_distance_part_1(instructions: &[Instruction]) -> i32 {
    let mut x = 0; // East-West position
    let mut y = 0; // North-South position
    let mut direction = 90; // 0 = North, 90 = East, 180 = South, 270 = West

    for instruction in instructions {
        let mut action = &instruction.action;
        let value = instruction.value;

        if let Action::Forward = action {
            action = match direction {
                0 => &Action::North,
                90 => &Action::East,
                180 => &Action::South,
                270 => &Action::West,
                _ => panic!("Invalid direction"),
            };
        }

        match action {
            Action::North => y += value,
            Action::South => y -= value,
            Action::East => x += value,
            Action::West => x -= value,
            Action::Left => direction = (direction - value + 360) % 360,
            Action::Right => direction = (direction + value) % 360,
            _ => {} // Action::Forward is handled before the match
        }
    }

    x.abs() + y.abs()
}

fn calculate_manhattan_distance_part_2(instructions: &[Instruction]) -> i32 {
    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for instruction in instructions {
        match instruction.action {
            Action::North => waypoint_y += instruction.value,
            Action::South => waypoint_y -= instruction.value,
            Action::East => waypoint_x += instruction.value,
            Action::West => waypoint_x -= instruction.value,
            Action::Left => {
                for _ in 0..(instruction.value / 90) {
                    let temp = waypoint_x;
                    waypoint_x = -waypoint_y;
                    waypoint_y = temp;
                }
            }
            Action::Right => {
                for _ in 0..(instruction.value / 90) {
                    let temp = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -temp;
                }
            }
            Action::Forward => {
                ship_x += waypoint_x * instruction.value;
                ship_y += waypoint_y * instruction.value;
            }
        }
    }

    ship_x.abs() + ship_y.abs()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let instructions = parse_input(&input);
    let distance = calculate_manhattan_distance_part_1(&instructions);
    distance
}

fn part_2(input: aoc::Input) -> impl ToString {
    let instructions = parse_input(&input);
    let distance = calculate_manhattan_distance_part_2(&instructions);
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_manhattan_distance_part_1() {
        let instructions = vec![
            Instruction {
                action: Action::Forward,
                value: 10,
            },
            Instruction {
                action: Action::North,
                value: 3,
            },
            Instruction {
                action: Action::Forward,
                value: 7,
            },
            Instruction {
                action: Action::Right,
                value: 90,
            },
            Instruction {
                action: Action::Forward,
                value: 11,
            },
        ];
        assert_eq!(calculate_manhattan_distance_part_1(&instructions), 25);
    }

    #[test]
    fn test_calculate_manhattan_distance_part_2() {
        let instructions = vec![
            Instruction {
                action: Action::Forward,
                value: 10,
            },
            Instruction {
                action: Action::North,
                value: 3,
            },
            Instruction {
                action: Action::Forward,
                value: 7,
            },
            Instruction {
                action: Action::Right,
                value: 90,
            },
            Instruction {
                action: Action::Forward,
                value: 11,
            },
        ];
        assert_eq!(calculate_manhattan_distance_part_2(&instructions), 286);
    }
}
