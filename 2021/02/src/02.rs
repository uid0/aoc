aoc::parts!(1,2);

fn part_1(input: aoc::Input) -> impl ToString {
    let (f, d) = input.raw()
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(f, d), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d),
                ("down", v) => (f, d + v),
                ("up", v) => (f, d - v),
                _ => unreachable!(),
            }
        });
    let output = f*d;
    output
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (f, d, _) = input.raw()
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(f, d, a), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d + a * v, a),
                ("down", v) => (f, d, a + v),
                ("up", v) => (f, d, a - v),
                _ => unreachable!(),
            }
        });

    let output = f*d;
    output

}