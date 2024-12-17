aoc::parts!(1, 2);

use aoc::Input;
use regex::Regex;

fn run_program_part1(program: &[u8], mut a: i64, mut b: i64, mut c: i64) -> String {
    let mut ip = 0usize;
    let mut output_values = std::collections::VecDeque::new(); // Use VecDeque for part 1

    fn literal(val: u8) -> i64 {
        val as i64
    }

    fn combo(val: u8, a: i64, b: i64, c: i64) -> i64 {
        match val {
            0..=3 => val as i64,
            4 => a,
            5 => b,
            6 => c,
            7 => panic!("Invalid combo operand"),
            _ => unreachable!(),
        }
    }

    while ip + 1 < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];

        match opcode {
            0 => {
                let denom_power = combo(operand, a, b, c);
                let denominator = 2i64.pow(denom_power as u32);
                a = a / denominator;
                ip += 2;
            }
            1 => {
                let val = literal(operand);
                b = b ^ val;
                ip += 2;
            }
            2 => {
                let val = combo(operand, a, b, c) % 8;
                b = val;
                ip += 2;
            }
            3 => {
                if a != 0 {
                    ip = literal(operand) as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                b = b ^ c;
                ip += 2;
            }
            5 => {
                let val = combo(operand, a, b, c) % 8;
                output_values.push_back(val);
                ip += 2;
            }
            6 => {
                let denom_power = combo(operand, a, b, c);
                let denominator = 2i64.pow(denom_power as u32);
                b = a / denominator;
                ip += 2;
            }
            7 => {
                let denom_power = combo(operand, a, b, c);
                let denominator = 2i64.pow(denom_power as u32);
                c = a / denominator;
                ip += 2;
            }
            _ => {
                break;
            }
        }

        if ip >= program.len() {
            break;
        }
    }

    let result: Vec<String> = output_values.into_iter().map(|v| v.to_string()).collect();
    result.join(",")
}

fn run_program_part2(program: &[u8], mut a: i64, mut b: i64, mut c: i64) -> String {
    let mut ip = 0usize;
    let mut output = String::new();

    while ip + 1 < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];

        let operand_val = match opcode {
            0 | 6 | 7 => match operand {
                0..=3 => operand as i64,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!("Invalid combo operand"),
            },
            1 | 3 => operand as i64,
            2 | 5 => match operand {
                0..=3 => operand as i64,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!("Invalid combo operand"),
            },
            4 => 0,
            _ => panic!("Invalid opcode"),
        };

        match opcode {
            0 => a /= 2i64.pow(operand_val as u32),
            1 => b ^= operand_val,
            2 => b = operand_val % 8,
            3 => {
                if a != 0 {
                    ip = operand_val as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => {
                if !output.is_empty() {
                    output.push(',');
                }
                output.push_str(&(operand_val % 8).to_string());
            }
            6 => b /= 2i64.pow(operand_val as u32),
            7 => c /= 2i64.pow(operand_val as u32),
            _ => break, // Exit loop on invalid opcode
        }

        ip += 2;
        if ip >= program.len() {
            break;
        }
    }

    output
}

fn parse_input(input_str: &str) -> (i64, i64, i64, Vec<u8>) {
    // ... (parsing code remains the same)
    let re_a = Regex::new(r"Register A: (\d+)").unwrap();
    let re_b = Regex::new(r"Register B: (\d+)").unwrap();
    let re_c = Regex::new(r"Register C: (\d+)").unwrap();
    let re_program = Regex::new(r"Program: ([\d,]+)").unwrap();

    let a: i64 = re_a.captures(input_str).unwrap()[1].parse().unwrap();
    let b: i64 = re_b.captures(input_str).unwrap()[1].parse().unwrap();
    let c: i64 = re_c.captures(input_str).unwrap()[1].parse().unwrap();

    let program_str = &re_program.captures(input_str).unwrap()[1];
    let program: Vec<u8> = program_str.split(',').map(|x| x.parse().unwrap()).collect();

    (a, b, c, program)
}

fn part_1(input: Input) -> impl ToString {
    let input_str = input.raw();
    let (a, b, c, program) = parse_input(input_str);
    run_program_part1(&program, a, b, c) // Call the original version
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = extended_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

fn mod_inverse(x: i64, m: i64) -> i64 {
    let (g, x_inv, _) = extended_gcd(x, m);
    if g != 1 {
        panic!("No modular inverse exists (modulus not coprime)");
    }
    (x_inv % m + m) % m
}

fn crt_solve(equations: &[(i64, i64)]) -> i64 {
    let mut n_prod: i64 = 1;
    for &(_, m) in equations {
        n_prod *= m;
    }

    let mut result: i64 = 0;
    for &(r, m) in equations {
        let n_i = n_prod / m;
        let inv = mod_inverse(n_i, m);
        result += r * n_i * inv;
    }

    result % n_prod
}

fn part_2(input: Input) -> impl ToString {
    let input_str = input.raw();
    let (_, b, c, program) = parse_input(input_str);
    let target = program
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let mut equations: Vec<(i64, i64)> = Vec::new();
    for (i, &opcode) in program.iter().enumerate() {
        if opcode == 5 && i + 1 < program.len() {
            let operand = program[i + 1];
            if operand == 4 {
                equations.push((program[i + 2] as i64 % 8, 8));
            }
        }
    }
    if equations.is_empty() {
        for a_candidate in 0..8 {
            let a = a_candidate;
            let output = run_program_part2(&program, a, b, c);
            if output == target {
                return a;
            }
        }
        panic!("No solution found within the search range.");
    }
    let a = crt_solve(&equations);

    let output = run_program_part2(&program, a, b, c);
    if output == target {
        a
    } else {
        //If the CRT doesn't provide a solution, we need to search from our starting point up until we find a solution.
        for test_a in (a..100000000).step_by(8) {
            let output = run_program_part2(&program, test_a, b, c);
            if output == target {
                return test_a;
            }
        }
        panic!("No solution found within the search range.");
    }
}

#[test]
fn test_part_1_examples() {
    assert_eq!(run_program_part1(&[2, 6], 0, 0, 9), "");
    assert_eq!(run_program_part1(&[5, 0, 5, 1, 5, 4], 10, 0, 0), "0,1,2");
    assert_eq!(
        run_program_part1(&[0, 1, 5, 4, 3, 0], 2024, 0, 0),
        "4,2,5,6,7,7,7,7,3,1,0"
    );
    assert_eq!(run_program_part1(&[1, 7], 0, 29, 0), "");
    assert_eq!(run_program_part1(&[4, 0], 0, 2024, 43690), "");
    assert_eq!(
        run_program_part1(&[0, 1, 5, 4, 3, 0], 729, 0, 0),
        "4,6,3,5,6,3,5,2,1,0"
    );
}

#[test]
fn test_part_2_example() {
    let input = "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0";
}
