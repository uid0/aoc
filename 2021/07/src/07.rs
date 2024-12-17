aoc::parts!(1,2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut subs = input.raw()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    let mid = subs.len() / 2;
    let med = *subs.select_nth_unstable(mid).1;

    let result = subs.iter().map(|n| (n - med).abs()).sum::<i32>();
    result
}

fn part_2(input: aoc::Input) -> impl ToString {
    let subs = input.raw()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    let result = (subs.iter().sum::<i32>() / subs.len() as i32..)
            .take(2)
            .map(|t| {
                subs.iter()
                    .map(|n| {
                        let d = (n - t).abs();
                        d * (d + 1) / 2
                    })
                    .sum::<i32>()
            })
            .min()
            .unwrap();

    result
}