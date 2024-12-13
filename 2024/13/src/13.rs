#[derive(Debug)]
struct Button {
    x: i32,
    y: i32,
    cost: i32,
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_x: i64,
    prize_y: i64,
}

fn parse_input(input: &str, offset: i64) -> Vec<Machine> {
    let mut machines = Vec::new();
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());

    while let Some(button_a_line) = lines.next() {
        let button_b_line = lines.next().expect("Expected Button B line");
        let prize_line = lines.next().expect("Expected Prize line");

        let button_a_parts: Vec<i32> = button_a_line
            .replace("Button A:", "")
            .replace("X+", "")
            .replace("Y+", "")
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let button_b_parts: Vec<i32> = button_b_line
            .replace("Button B:", "")
            .replace("X+", "")
            .replace("Y+", "")
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let prize_parts: Vec<i64> = prize_line
            .replace("Prize:", "")
            .replace("X=", "")
            .replace("Y=", "")
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if button_a_parts.len() != 2 || button_b_parts.len() != 2 || prize_parts.len() != 2 {
            panic!(
                "Invalid input format: {}, {}, {}",
                button_a_line, button_b_line, prize_line
            );
        }

        machines.push(Machine {
            button_a: Button {
                x: button_a_parts[0],
                y: button_a_parts[1],
                cost: 3,
            },
            button_b: Button {
                x: button_b_parts[0],
                y: button_b_parts[1],
                cost: 1,
            },
            prize_x: prize_parts[0] + offset,
            prize_y: prize_parts[1] + offset,
        });
    }

    machines
}

fn solve_machine(machine: &Machine) -> Option<i64> {
    let ax = machine.button_a.x as i64;
    let ay = machine.button_a.y as i64;
    let bx = machine.button_b.x as i64;
    let by = machine.button_b.y as i64;
    let px = machine.prize_x;
    let py = machine.prize_y;

    let determinant = ax * by - ay * bx;
    if determinant == 0 {
        return None; // No solution or infinite solutions
    }

    let m = (px * by - py * bx) / determinant;
    if m * determinant != px * by - py * bx {
        return None; // m is not an integer
    }

    let n = (py - ay * m) / by;
    if n * by != py - ay * m {
        return None; // n is not an integer
    }

    if m < 0 || n < 0 {
        return None; // No negative presses allowed
    }

    Some(3 * m + n) // Total cost
}

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let raw_data = input.raw();
    let machines = parse_input(&raw_data, 0);

    let (prizes_won, total_cost) = machines.iter().fold((0, 0), |(prizes, cost), machine| {
        if let Some(min_cost) = solve_machine(machine) {
            (prizes + 1, cost + min_cost)
        } else {
            (prizes, cost)
        }
    });

    total_cost
}

fn part_2(input: aoc::Input) -> impl ToString {
    let raw_data = input.raw();
    let machines = parse_input(&raw_data, 10000000000000);

    let (prizes_won, total_cost) = machines.iter().fold((0, 0), |(prizes, cost), machine| {
        if let Some(min_cost) = solve_machine(machine) {
            (prizes + 1, cost + min_cost)
        } else {
            (prizes, cost)
        }
    });

    total_cost
}

mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_1() {
        let input =
            ("Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\n".to_string(),);

        let result = part_1(input);
        assert_eq!(result.to_string(), "2 prizes won with a total cost of 480");
    }

    #[test]
    fn test_example_input_part_2() {
        let input =
            ("Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\n".to_string(),);

        let result = part_2(input);
        assert!(result.to_string().contains("prizes won"));
    }
}
