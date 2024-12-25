aoc::parts!(1, 2);

use regex::Regex;
use std::collections::HashSet;

fn is_valid_passport(passport: &str) -> bool {
    let required_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    let mut present_fields = HashSet::new();

    for field in passport.split_whitespace() {
        let key_value: Vec<&str> = field.split(':').collect();
        if key_value.len() == 2 {
            present_fields.insert(key_value[0]);
        }
    }

    required_fields.is_subset(&present_fields)
}

fn is_valid_passport_part_2(passport: &str) -> bool {
    if !is_valid_passport(passport) {
        return false;
    }

    for field in passport.split_whitespace() {
        let key_value: Vec<&str> = field.split(':').collect();
        if key_value.len() != 2 {
            return false;
        }
        let key = key_value[0];
        let value = key_value[1];

        match key {
            "byr" => {
                if let Ok(year) = value.parse::<u32>() {
                    if !(1920..=2002).contains(&year) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "iyr" => {
                if let Ok(year) = value.parse::<u32>() {
                    if !(2010..=2020).contains(&year) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "eyr" => {
                if let Ok(year) = value.parse::<u32>() {
                    if !(2020..=2030).contains(&year) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hgt" => {
                if let Some(captures) = Regex::new(r"^(\d+)(cm|in)$").unwrap().captures(value) {
                    if let Ok(height) = captures[1].parse::<u32>() {
                        match &captures[2] {
                            "cm" => {
                                if !(150..=193).contains(&height) {
                                    return false;
                                }
                            }
                            "in" => {
                                if !(59..=76).contains(&height) {
                                    return false;
                                }
                            }
                            _ => return false,
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                if !Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(value) {
                    return false;
                }
            }
            "ecl" => {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                    return false;
                }
            }
            "pid" => {
                if !Regex::new(r"^\d{9}$").unwrap().is_match(value) {
                    return false;
                }
            }
            "cid" => {}
            _ => return false,
        }
    }

    true
}

fn part_1(input: aoc::Input) -> impl ToString {
    let valid_passports = input
        .raw()
        .split("\n\n")
        .filter(|passport| is_valid_passport(passport))
        .count();

    valid_passports
}

fn part_2(input: aoc::Input) -> impl ToString {
    let valid_passports = input
        .raw()
        .split("\n\n")
        .filter(|passport| is_valid_passport_part_2(passport))
        .count();

    valid_passports
}

#[cfg(test)]
mod tests {
    use super::*;

    // ... (Existing tests for part_1) ...
    #[test]
    fn test_valid_passport() {
        let passport =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(is_valid_passport(passport));
    }

    #[test]
    fn test_invalid_passport_missing_hgt() {
        let passport = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        assert!(!is_valid_passport(passport));
    }

    #[test]
    fn test_valid_passport_missing_cid() {
        let passport = "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm";
        assert!(is_valid_passport(passport));
    }

    #[test]
    fn test_invalid_passport_missing_byr_and_cid() {
        let passport = "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in";
        assert!(!is_valid_passport(passport));
    }

    #[test]
    fn test_part_1_example() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
        let passports: Vec<&str> = input.split("\n\n").collect();
        let valid_count: usize = passports
            .iter()
            .filter(|p| is_valid_passport(p))
            .map(|_| 1)
            .sum();
        assert_eq!(valid_count, 2);
    }

    #[test]
    fn test_invalid_passports_part_2() {
        let inputs = [
            "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007",
        ];
        for input in inputs.iter() {
            assert!(!is_valid_passport_part_2(input));
        }
    }

    #[test]
    fn test_valid_passports_part_2() {
        let inputs = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];
        for input in inputs.iter() {
            assert!(is_valid_passport_part_2(input));
        }
    }

    #[test]
    fn test_part_2_example() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passports: Vec<&str> = input.split("\n\n").collect();
        let valid_count: usize = passports
            .iter()
            .filter(|p| is_valid_passport_part_2(p))
            .map(|_| 1)
            .sum();
        assert_eq!(valid_count, 4);
    }
}
