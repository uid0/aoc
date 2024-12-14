aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let appinput = input.raw();
    let depths = appinput
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>();

    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i - 1] < depths[i] {
            count += 1;
        }
    }

    count
}

fn part_2(input: aoc::Input) -> impl ToString {
    let depths: Vec<u16> = input.raw().lines().map(|n| n.parse().unwrap()).collect();

    let mut count = 0;
    for i in 3..depths.len() {
        if depths[i - 3] < depths[i] {
            count += 1;
        }
    }

    count
}
