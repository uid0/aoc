use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

aoc::parts!(1, 2);

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

fn part_2(input: aoc::Input) -> impl ToString {
    let sections: Vec<Vec<&str>> = input
        .as_lines()
        .into_iter()
        .copied()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
        .map(|s| s.to_vec())
        .collect();
    let rules = parse_rules(sections[0].join("\n").as_str());
    let my_ticket = parse_tickets(sections[1].join("\n").as_str())
        .first()
        .unwrap()
        .clone();
    let nearby_tickets = parse_tickets(sections[2].join("\n").as_str());

    // Filter out invalid tickets
    let valid_tickets: Vec<Vec<u32>> = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&value| is_valid_for_any_rule(value, &rules))
        })
        .collect();

    // Solve for field mappings
    let field_mapping = solve_part_2(&rules, &valid_tickets);

    // Calculate product of departure fields on my ticket
    let mut product = 1;
    for (field_name, index) in field_mapping {
        if field_name.starts_with("departure") {
            product *= my_ticket[index] as u64;
        }
    }

    product
}

fn solve_part_2(rules: &[Rule], valid_tickets: &[Vec<u32>]) -> HashMap<String, usize> {
    let num_fields = valid_tickets[0].len();
    let mut possible_fields: Vec<HashSet<String>> = vec![HashSet::new(); num_fields];

    // Initialize possible fields for each position
    for i in 0..num_fields {
        for rule in rules {
            possible_fields[i].insert(rule.name.clone());
        }
    }

    // Eliminate impossible fields
    for ticket in valid_tickets {
        for (i, &value) in ticket.iter().enumerate() {
            for rule in rules {
                if !rule.ranges.iter().any(|range| range.contains(&value)) {
                    possible_fields[i].remove(&rule.name);
                }
            }
        }
    }

    // Deduce field order
    let mut field_mapping: HashMap<String, usize> = HashMap::new();
    while field_mapping.len() < num_fields {
        for i in 0..num_fields {
            if possible_fields[i].len() == 1 {
                let field_name = possible_fields[i].iter().next().unwrap().clone();
                field_mapping.insert(field_name.clone(), i);
                for j in 0..num_fields {
                    possible_fields[j].remove(&field_name);
                }
            }
        }
    }

    field_mapping
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

fn parse_rules(rules_str: &str) -> Vec<Rule> {
    rules_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let name = parts[0].to_string();
            let ranges_str: Vec<&str> = parts[1].split(" or ").collect();
            let ranges = ranges_str
                .iter()
                .map(|range_str| {
                    let range_parts: Vec<u32> =
                        range_str.split('-').map(|s| s.parse().unwrap()).collect();
                    range_parts[0]..=range_parts[1]
                })
                .collect();
            Rule { name, ranges }
        })
        .collect()
}

fn parse_tickets(tickets_str: &str) -> Vec<Vec<u32>> {
    tickets_str
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn is_valid_for_any_rule(value: u32, rules: &[Rule]) -> bool {
    rules
        .iter()
        .any(|rule| rule.ranges.iter().any(|range| range.contains(&value)))
}

fn part_1(input: aoc::Input) -> impl ToString {
    let sections: Vec<Vec<&str>> = input
        .as_lines()
        .into_iter()
        .copied()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
        .map(|s| s.to_vec())
        .collect();
    let rules = parse_rules(sections[0].join("\n").as_str());
    let nearby_tickets = parse_tickets(sections[2].join("\n").as_str());

    let mut error_rate = 0;
    for ticket in nearby_tickets {
        for value in ticket {
            if !is_valid_for_any_rule(value, &rules) {
                error_rate += value;
            }
        }
    }

    error_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
        let sections: Vec<Vec<&str>> = input
            .split("\n\n")
            .map(|s| s.lines().collect::<Vec<&str>>())
            .collect();
        let rules = parse_rules(sections[0].join("\n").as_str());
        let nearby_tickets = parse_tickets(sections[2].join("\n").as_str());

        let mut error_rate = 0;
        for ticket in nearby_tickets {
            for value in ticket {
                if !is_valid_for_any_rule(value, &rules) {
                    error_rate += value;
                }
            }
        }

        assert_eq!(error_rate, 71);
    }

    #[test]
    fn test_part_2() {
        let input = "class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9";
        let sections: Vec<Vec<&str>> = input
            .split("\n\n")
            .map(|s| s.lines().collect::<Vec<&str>>())
            .collect();
        let rules = parse_rules(sections[0].join("\n").as_str());
        let my_ticket = parse_tickets(sections[1].join("\n").as_str())
            .first()
            .unwrap()
            .clone();
        let nearby_tickets = parse_tickets(sections[2].join("\n").as_str());

        let valid_tickets: Vec<Vec<u32>> = nearby_tickets
            .into_iter()
            .filter(|ticket| {
                ticket
                    .iter()
                    .all(|&value| is_valid_for_any_rule(value, &rules))
            })
            .collect();

        let field_mapping = solve_part_2(&rules, &valid_tickets);

        assert_eq!(field_mapping["row"], 0);
        assert_eq!(field_mapping["class"], 1);
        assert_eq!(field_mapping["seat"], 2);
    }
}
