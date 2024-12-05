use std::collections::HashMap;

aoc::parts!(1, 2);
fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let ordering_rules = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line.split('|').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let updates = sections[1]
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (ordering_rules, updates)
}

fn is_update_valid(ordering_rules: &[(u32, u32)], update: &[u32]) -> bool {
    let position: HashMap<u32, usize> = update.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    for &(x, y) in ordering_rules {
        if let (Some(&pos_x), Some(&pos_y)) = (position.get(&x), position.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }

    true
}

fn reorder_update(ordering_rules: &[(u32, u32)], update: &mut Vec<u32>) {
    update.sort_by(|&a, &b| {
        for &(x, y) in ordering_rules {
            if x == a && y == b {
                return std::cmp::Ordering::Less;
            } else if x == b && y == a {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    });
}

fn find_middle_number(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn part_1(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let (ordering_rules, updates) = parse_input(&raw_input);

    let mut middle_sum = 0;

    for update in updates {
        if is_update_valid(&ordering_rules, &update) {
            middle_sum += find_middle_number(&update);
        }
    }

    middle_sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let raw_input = input.raw();
    let (ordering_rules, mut updates) = parse_input(&raw_input);

    let mut middle_sum = 0;

    for update in updates.iter_mut() {
        if !is_update_valid(&ordering_rules, update) {
            reorder_update(&ordering_rules, update);
            middle_sum += find_middle_number(update);
        }
    }

    middle_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let (ordering_rules, updates) = parse_input(input);

        let valid_updates: Vec<_> = updates
            .iter()
            .filter(|update| is_update_valid(&ordering_rules, update))
            .cloned()
            .collect();

        let middle_numbers: Vec<u32> = valid_updates
            .iter()
            .map(|update| find_middle_number(update))
            .collect();

        assert_eq!(middle_numbers, vec![61, 53, 29]);
        assert_eq!(middle_numbers.iter().sum::<u32>(), 143);
    }

    #[test]
    fn test_reorder_case() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let (ordering_rules, mut updates) = parse_input(input);

        let invalid_updates: Vec<_> = updates
            .iter_mut()
            .filter(|update| !is_update_valid(&ordering_rules, update))
            .collect::<Vec<_>>();

        for update in invalid_updates {
            reorder_update(&ordering_rules, update);
        }

        let reordered_middle_numbers: Vec<u32> = updates
            .iter()
            .filter(|update| !is_update_valid(&ordering_rules, update))
            .map(|update| find_middle_number(update))
            .collect();

        assert_eq!(reordered_middle_numbers, vec![47, 29, 47]);
        assert_eq!(reordered_middle_numbers.iter().sum::<u32>(), 123);
    }
}
