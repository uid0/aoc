use regex::Regex;

aoc::parts!(1,2);

fn part_1(input: aoc::Input) -> impl ToString {

    let memory_text = input.raw(); // or appropriate method to get &str

    // Define a regex to match valid `mul(X,Y)` patterns
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Extract all valid `mul(X,Y)` instructions and compute the results
    let sum: i32 = re
        .captures_iter(memory_text)
        .map(|caps| {
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            x * y
        })
        .sum();

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    // Decode raw input into a UTF-8 string
    let memory_text = input.raw(); // or appropriate method to get &str

    // Define a regex to match relevant instructions
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut is_enabled = true; // Multiplications start enabled by default

    for caps in re.captures_iter(memory_text) {
        if let Some(mul_caps) = caps.get(0) {
            let matched = mul_caps.as_str();

            // Check for `mul(X,Y)`
            if matched.starts_with("mul") {
                if is_enabled {
                    let x: i32 = caps[1].parse().unwrap();
                    let y: i32 = caps[2].parse().unwrap();
                    sum += x * y;
                }
            }
            // Check for `do()`
            else if matched == "do()" {
                is_enabled = true;
            }
            // Check for `don't()`
            else if matched == "don't()" {
                is_enabled = false;
            }
        }
    }

    sum
}