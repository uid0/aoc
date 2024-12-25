aoc::parts!(1, 2);

use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let operation = parts[0];
    let argument = parts[1].parse().unwrap();

    match operation {
        "acc" => Instruction::Acc(argument),
        "jmp" => Instruction::Jmp(argument),
        "nop" => Instruction::Nop(argument),
        _ => panic!("Invalid operation: {}", operation),
    }
}

fn run_program(program: &[Instruction]) -> (i32, bool) {
    let mut accumulator = 0;
    let mut pc = 0; // Program counter
    let mut visited = HashSet::new();
    let mut terminated = false;

    while pc < program.len() {
        if visited.contains(&pc) {
            return (accumulator, false); // Infinite loop detected
        }
        visited.insert(pc);

        match program[pc] {
            Instruction::Acc(val) => {
                accumulator += val;
                pc += 1;
            }
            Instruction::Jmp(offset) => {
                pc = (pc as i32 + offset) as usize;
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }
    }

    (accumulator, true) // Program terminated normally
}

fn part_1(input: aoc::Input) -> impl ToString {
    let program: Vec<Instruction> = input.raw().lines().map(parse_instruction).collect();
    let (accumulator_value, _) = run_program(&program);
    accumulator_value
}

fn part_2(input: aoc::Input) -> impl ToString {
    let original_program: Vec<Instruction> = input.raw().lines().map(parse_instruction).collect();

    for i in 0..original_program.len() {
        let mut modified_program = original_program.clone();
        match modified_program[i] {
            Instruction::Jmp(val) => modified_program[i] = Instruction::Nop(val),
            Instruction::Nop(val) => modified_program[i] = Instruction::Jmp(val),
            _ => continue, // Skip if it's an Acc instruction
        }

        let (accumulator, terminated) = run_program(&modified_program);
        if terminated {
            return accumulator.to_string();
        }
    }

    panic!("No solution found"); // Should ideally not reach here with valid input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert!(matches!(parse_instruction("acc +1"), Instruction::Acc(1)));
        assert!(matches!(parse_instruction("jmp -4"), Instruction::Jmp(-4)));
        assert!(matches!(parse_instruction("nop +0"), Instruction::Nop(0)));
    }

    #[test]
    fn test_run_program_infinite_loop() {
        let program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];
        let (accumulator, terminated) = run_program(&program);
        assert_eq!(accumulator, 5);
        assert!(!terminated);
    }

    #[test]
    fn test_part_1_example() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let program: Vec<Instruction> = input.lines().map(parse_instruction).collect();
        let (accumulator, _) = run_program(&program);
        assert_eq!(accumulator, 5);
    }

    #[test]
    fn test_part_2_example() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let original_program: Vec<Instruction> = input.lines().map(parse_instruction).collect();

        for i in 0..original_program.len() {
            let mut modified_program = original_program.clone();
            match modified_program[i] {
                Instruction::Jmp(val) => modified_program[i] = Instruction::Nop(val),
                Instruction::Nop(val) => modified_program[i] = Instruction::Jmp(val),
                _ => continue,
            }

            let (accumulator, terminated) = run_program(&modified_program);
            if terminated {
                assert_eq!(accumulator, 8);
                return;
            }
        }
    }
}
