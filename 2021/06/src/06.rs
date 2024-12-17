aoc::parts!(1,2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut map = input.raw()
        .split(',')
        .fold([0; 9], |mut map, n| {
            map[n.parse::<usize>().unwrap()] += 1;
            map
        });

    (1..80).for_each(|day| map[(day + 7) % 9] += map[day % 9]);

    let result = map.iter().sum::<usize>();
    result
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut map = input.raw()
        .split(',')
        .fold([0; 9], |mut map, n| {
            map[n.parse::<usize>().unwrap()] += 1;
            map
        });

    (1..256).for_each(|day| map[(day + 7) % 9] += map[day % 9]);

    let result = map.iter().sum::<usize>();
    result
}