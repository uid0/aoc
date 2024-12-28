aoc::parts!(1, 2);

use std::collections::HashMap;

fn apply_mask_v2(address: u64, mask: &str) -> Vec<u64> {
    let mut addresses = vec![0];
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '1' => {
                for addr in &mut addresses {
                    *addr |= 1 << i;
                }
            }
            'X' => {
                let mut new_addresses = Vec::new();
                for addr in &addresses {
                    let mut new_addr = *addr;
                    new_addr &= !(1 << i);
                    new_addresses.push(new_addr);
                    let mut new_addr2 = *addr;
                    new_addr2 |= 1 << i;
                    new_addresses.push(new_addr2);
                }
                addresses = new_addresses;
            }
            _ => {
                for addr in &mut addresses {
                    if (address >> i) & 1 == 1 {
                        *addr |= 1 << i;
                    } else {
                        *addr &= !(1 << i);
                    }
                }
            }
        }
    }
    addresses
}

fn apply_mask(value: u64, mask: &str) -> u64 {
    let mut result = value;
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '1' => result |= 1 << i,
            '0' => result &= !(1 << i),
            _ => {}
        }
    }
    result
}

fn part_1(input: aoc::Input) -> impl ToString {
    let lines: Vec<&str> = input.as_lines().into_iter().copied().collect();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = "";

    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts[0] == "mask" {
            current_mask = parts[1];
        } else {
            let address = parts[0][4..parts[0].len() - 1].parse::<u64>().unwrap();
            let value = parts[1].parse::<u64>().unwrap();
            let masked_value = apply_mask(value, current_mask);
            memory.insert(address, masked_value);
        }
    }

    memory.values().sum::<u64>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let lines: Vec<&str> = input.as_lines().into_iter().copied().collect();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = "";

    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts[0] == "mask" {
            current_mask = parts[1];
        } else {
            let address = parts[0][4..parts[0].len() - 1].parse::<u64>().unwrap();
            let value = parts[1].parse::<u64>().unwrap();
            let masked_addresses = apply_mask_v2(address, current_mask);
            for masked_address in masked_addresses {
                memory.insert(masked_address, value);
            }
        }
    }

    memory.values().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        assert_eq!(apply_mask(11, mask), 73);
        assert_eq!(apply_mask(101, mask), 101);
        assert_eq!(apply_mask(0, mask), 64);
    }

    #[test]
    fn test_part_1() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let lines: Vec<&str> = input.split('\n').collect();
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut current_mask = "";
        for line in lines {
            let parts: Vec<&str> = line.split(" = ").collect();
            if parts[0] == "mask" {
                current_mask = parts[1];
            } else {
                let address = parts[0][4..parts[0].len() - 1].parse::<u64>().unwrap();
                let value = parts[1].parse::<u64>().unwrap();
                let masked_value = apply_mask(value, current_mask);
                memory.insert(address, masked_value);
            }
        }
        assert_eq!(memory.values().sum::<u64>(), 165);
    }

    #[test]
    fn test_apply_mask_v2() {
        let mask = "000000000000000000000000000000X1001X";
        let addresses = apply_mask_v2(42, mask);
        assert!(addresses.contains(&26));
        assert!(addresses.contains(&27));
        assert!(addresses.contains(&58));
        assert!(addresses.contains(&59));

        let mask = "00000000000000000000000000000000X0XX";
        let addresses = apply_mask_v2(26, mask);
        assert!(addresses.contains(&16));
        assert!(addresses.contains(&17));
        assert!(addresses.contains(&18));
        assert!(addresses.contains(&19));
        assert!(addresses.contains(&24));
        assert!(addresses.contains(&25));
        assert!(addresses.contains(&26));
        assert!(addresses.contains(&27));
    }

    #[test]
    fn test_part_2() {
        let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        let lines: Vec<&str> = input.split('\n').collect();
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut current_mask = "";
        for line in lines {
            let parts: Vec<&str> = line.split(" = ").collect();
            if parts[0] == "mask" {
                current_mask = parts[1];
            } else {
                let address = parts[0][4..parts[0].len() - 1].parse::<u64>().unwrap();
                let value = parts[1].parse::<u64>().unwrap();
                let masked_addresses = apply_mask_v2(address, current_mask);
                for masked_address in masked_addresses {
                    memory.insert(masked_address, value);
                }
            }
        }
        assert_eq!(memory.values().sum::<u64>(), 208);
    }
}
