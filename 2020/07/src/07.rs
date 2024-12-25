aoc::parts!(1, 2);

use regex::Regex;
use std::collections::{HashMap, HashSet};

fn can_contain_shiny_gold(
    rules: &HashMap<String, Vec<(usize, String)>>,
    color: &str,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if let Some(&result) = cache.get(color) {
        return result;
    }

    if color == "shiny gold" {
        return true;
    }

    if let Some(contents) = rules.get(color) {
        let result = contents
            .iter()
            .any(|(_, contained_color)| can_contain_shiny_gold(rules, contained_color, cache));
        cache.insert(color.to_string(), result);
        return result;
    }

    false
}

fn count_contained_bags(rules: &HashMap<String, Vec<(usize, String)>>, color: &str) -> usize {
    let mut total_bags = 0;

    if let Some(contents) = rules.get(color) {
        for (count, contained_color) in contents {
            total_bags += count; // Add the direct count of contained bags
            total_bags += count * count_contained_bags(rules, contained_color); // Add bags inside those
        }
    }

    total_bags
}

fn part_1(input: aoc::Input) -> impl ToString {
    let bag_rules_regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
    let contained_bag_regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    let mut rules = HashMap::new();

    for line in input.raw().lines() {
        let captures = bag_rules_regex.captures(line).unwrap();
        let bag_color = captures[1].to_string();
        let contents_str = &captures[2];

        let mut contents = Vec::new();
        if contents_str != "no other bags." {
            for contained_bag_capture in contained_bag_regex.captures_iter(contents_str) {
                let count = contained_bag_capture[1].parse().unwrap();
                let contained_color = contained_bag_capture[2].to_string();
                contents.push((count, contained_color));
            }
        }

        rules.insert(bag_color, contents);
    }

    let mut cache = HashMap::new();
    let count = rules
        .keys()
        .filter(|&color| color != "shiny gold" && can_contain_shiny_gold(&rules, color, &mut cache))
        .count();

    count
}

fn part_2(input: aoc::Input) -> impl ToString {
    let bag_rules_regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
    let contained_bag_regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    let mut rules = HashMap::new();

    for line in input.raw().lines() {
        let captures = bag_rules_regex.captures(line).unwrap();
        let bag_color = captures[1].to_string();
        let contents_str = &captures[2];

        let mut contents = Vec::new();
        if contents_str != "no other bags." {
            for contained_bag_capture in contained_bag_regex.captures_iter(contents_str) {
                let count = contained_bag_capture[1].parse().unwrap();
                let contained_color = contained_bag_capture[2].to_string();
                contents.push((count, contained_color));
            }
        }

        rules.insert(bag_color, contents);
    }

    count_contained_bags(&rules, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        let bag_rules_regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
        let contained_bag_regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        let mut rules = HashMap::new();

        for line in input.lines() {
            let captures = bag_rules_regex.captures(line).unwrap();
            let bag_color = captures[1].to_string();
            let contents_str = &captures[2];

            let mut contents = Vec::new();
            if contents_str != "no other bags." {
                for contained_bag_capture in contained_bag_regex.captures_iter(contents_str) {
                    let count = contained_bag_capture[1].parse().unwrap();
                    let contained_color = contained_bag_capture[2].to_string();
                    contents.push((count, contained_color));
                }
            }

            rules.insert(bag_color, contents);
        }

        let mut cache = HashMap::new();
        let count = rules
            .keys()
            .filter(|&color| {
                color != "shiny gold" && can_contain_shiny_gold(&rules, color, &mut cache)
            })
            .count();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        let bag_rules_regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
        let contained_bag_regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        let mut rules = HashMap::new();

        for line in input.lines() {
            let captures = bag_rules_regex.captures(line).unwrap();
            let bag_color = captures[1].to_string();
            let contents_str = &captures[2];

            let mut contents = Vec::new();
            if contents_str != "no other bags." {
                for contained_bag_capture in contained_bag_regex.captures_iter(contents_str) {
                    let count = contained_bag_capture[1].parse().unwrap();
                    let contained_color = contained_bag_capture[2].to_string();
                    contents.push((count, contained_color));
                }
            }

            rules.insert(bag_color, contents);
        }

        assert_eq!(count_contained_bags(&rules, "shiny gold"), 32);
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
        let bag_rules_regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
        let contained_bag_regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        let mut rules = HashMap::new();

        for line in input.lines() {
            let captures = bag_rules_regex.captures(line).unwrap();
            let bag_color = captures[1].to_string();
            let contents_str = &captures[2];

            let mut contents = Vec::new();
            if contents_str != "no other bags." {
                for contained_bag_capture in contained_bag_regex.captures_iter(contents_str) {
                    let count = contained_bag_capture[1].parse().unwrap();
                    let contained_color = contained_bag_capture[2].to_string();
                    contents.push((count, contained_color));
                }
            }

            rules.insert(bag_color, contents);
        }

        assert_eq!(count_contained_bags(&rules, "shiny gold"), 126);
    }
}
