aoc::parts!(1, 2);

use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

#[derive(Debug)]
struct PasswordEntry {
    policy: Policy,
    password: Vec<u8>,
}

impl FromStr for PasswordEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        let policy_parts: Vec<&str> = parts[0].split(' ').collect();
        let range_parts: Vec<&str> = policy_parts[0].split('-').collect();

        let policy = Policy {
            min: range_parts[0].parse().unwrap(),
            max: range_parts[1].parse().unwrap(),
            letter: policy_parts[1].chars().next().unwrap(),
        };

        Ok(PasswordEntry {
            policy,
            password: parts[1].as_bytes().to_vec(),
        })
    }
}

fn is_valid_password(entry: &PasswordEntry) -> bool {
    let count = entry
        .password
        .iter()
        .filter(|&&b| b == entry.policy.letter as u8)
        .count();
    count >= entry.policy.min && count <= entry.policy.max
}

fn is_valid_password_part_2(entry: &PasswordEntry) -> bool {
    let pos1_match = entry.password[entry.policy.min - 1] == entry.policy.letter as u8;
    let pos2_match = entry.password[entry.policy.max - 1] == entry.policy.letter as u8;
    pos1_match ^ pos2_match // XOR
}

fn part_1(input: aoc::Input) -> impl ToString {
    let valid_passwords = input
        .raw()
        .lines()
        .filter_map(|line| line.parse::<PasswordEntry>().ok())
        .filter(|entry| is_valid_password(entry))
        .count();

    valid_passwords
}

fn part_2(input: aoc::Input) -> impl ToString {
    let valid_passwords = input
        .raw()
        .lines()
        .filter_map(|line| line.parse::<PasswordEntry>().ok())
        .filter(|entry| is_valid_password_part_2(entry))
        .count();

    valid_passwords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_policy_parsing() {
        let input = "1-3 a: abcde";
        let entry = PasswordEntry::from_str(input).unwrap();
        assert_eq!(entry.policy.min, 1);
        assert_eq!(entry.policy.max, 3);
        assert_eq!(entry.policy.letter, 'a');
        assert_eq!(entry.password, b"abcde");
    }

    #[test]
    fn test_valid_password() {
        let entry = PasswordEntry {
            policy: Policy {
                min: 1,
                max: 3,
                letter: 'a',
            },
            password: b"abcde".to_vec(),
        };
        assert!(is_valid_password(&entry));
    }

    #[test]
    fn test_invalid_password_too_few() {
        let entry = PasswordEntry {
            policy: Policy {
                min: 2,
                max: 3,
                letter: 'a',
            },
            password: b"abcde".to_vec(),
        };
        assert!(!is_valid_password(&entry));
    }

    #[test]
    fn test_invalid_password_too_many() {
        let entry = PasswordEntry {
            policy: Policy {
                min: 1,
                max: 3,
                letter: 'c',
            },
            password: b"cccc".to_vec(),
        };
        assert!(!is_valid_password(&entry));
    }

    #[test]
    fn test_part_1_example() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let entries: Vec<PasswordEntry> = input.lines().map(|line| line.parse().unwrap()).collect();
        let valid_count: usize = entries
            .iter()
            .filter(|entry| is_valid_password(entry))
            .map(|_| 1)
            .sum();
        assert_eq!(valid_count, 2);
    }

    #[test]
    fn test_valid_password_part_2_case_1() {
        let entry = PasswordEntry {
            policy: Policy {
                min: 1,
                max: 3,
                letter: 'a',
            },
            password: b"abcde".to_vec(),
        };
        assert!(is_valid_password_part_2(&entry));
    }

    #[test]
    fn test_valid_password_part_2_case_2() {
        let entry = PasswordEntry {
            policy: Policy {
                min: 1,
                max: 3,
                letter: 'b',
            },
            password: b"cdefg".to_vec(),
        };
        assert!(!is_valid_password_part_2(&entry));
    }

    #[test]
    fn test_valid_password_part_2_case_3() {
        let entry = PasswordEntry {
            policy: Policy {
                min: 2,
                max: 9,
                letter: 'c',
            },
            password: b"ccccccccc".to_vec(),
        };
        assert!(!is_valid_password_part_2(&entry));
    }

    #[test]
    fn test_part_2_example() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let entries: Vec<PasswordEntry> = input.lines().map(|line| line.parse().unwrap()).collect();
        let valid_count: usize = entries
            .iter()
            .filter(|entry| is_valid_password_part_2(entry))
            .map(|_| 1)
            .sum();
        assert_eq!(valid_count, 1);
    }
}
