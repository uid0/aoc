use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Arc<dyn Fn(u64) -> u64 + Send + Sync>,
    test_divisor: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

impl Monkey {
    fn inspect_and_throw(&mut self, monkeys: &mut [Monkey], modulo: Option<u64>) {
        while let Some(item) = self.items.pop_front() {
            self.inspections += 1;
            let new_worry = if let Some(m) = modulo {
                (self.operation)(item) % m
            } else {
                (self.operation)(item) / 3
            };
            let target = if new_worry % self.test_divisor == 0 {
                self.if_true
            } else {
                self.if_false
            };
            monkeys[target].items.push_back(new_worry);
        }
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for block in input.split("\n\n") {
        let lines: Vec<_> = block.lines().collect();
        let items = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let operation_str = lines[2].strip_prefix("  Operation: new = old ").unwrap();
        let operation: Arc<dyn Fn(u64) -> u64 + Send + Sync> = if operation_str.starts_with('*') {
            if operation_str.trim() == "* old" {
                Arc::new(move |old| old * old)
            } else {
                let factor = operation_str[2..].parse::<u64>().unwrap();
                Arc::new(move |old| old * factor)
            }
        } else if operation_str.starts_with('+') {
            let addend = operation_str[2..].parse::<u64>().unwrap();
            Arc::new(move |old| old + addend)
        } else {
            panic!("Unexpected operation: {}", operation_str);
        };

        let test_divisor = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let if_true = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let if_false = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test_divisor,
            if_true,
            if_false,
            inspections: 0,
        });
    }

    monkeys
}

fn simulate(monkeys: &mut [Monkey], rounds: usize, modulo: Option<u64>) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            monkey.inspect_and_throw(monkeys, modulo);
            monkeys[i] = monkey;
        }
    }
}

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let mut monkeys = parse_input(&raw_input);
    simulate(&mut monkeys, 20, None); // No modulo for part 1

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}

fn part_2(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let mut monkeys = parse_input(&raw_input);

    // Calculate the least common multiple of all test divisors
    let modulo: u64 = monkeys.iter().map(|m| m.test_divisor).product();

    simulate(&mut monkeys, 10_000, Some(modulo)); // Use modulo for part 2

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}
