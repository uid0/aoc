aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    const WIDTH: usize = 12;
    const COUNT: usize = 1000;

    let gamma = input
        .raw()
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; WIDTH], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
        .sum::<u32>();

    let epsilon = !gamma & ((1 << WIDTH) - 1); // Calculate epsilon separately
    let result = gamma * epsilon; // Calculate the result

    println!("{}", result); // Use the calculated result
    let output = result; // Assign the calculated result
    output
}

fn part_2(input: aoc::Input) -> impl ToString {
    const WIDTH: usize = 12;

    let nums = input
        .raw()
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let mut oxy = nums.clone();
    for i in (0..WIDTH).rev() {
        let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
        oxy = oxy
            .into_iter()
            .filter(|n| (*n & 1 << i > 0) == one)
            .collect();
        if oxy.len() == 1 {
            break;
        }
    }
    let oxy = oxy[0];

    let mut co2 = nums;
    for i in (0..WIDTH).rev() {
        let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
        co2 = co2
            .into_iter()
            .filter(|n| (*n & 1 << i > 0) != one)
            .collect();
        if co2.len() == 1 {
            break;
        }
    }
    let co2 = co2[0];

    println!("{}", oxy * co2);
    let output = oxy * co2;
    output
}
