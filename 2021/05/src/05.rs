use itertools::Itertools;

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut map = vec![0u8; 1000 * 1000];
    let mut overlaps = 0;

    for line in input.lines() {
        let (mut p1, mut p2) = line
            .split_once(" -> ")
            .unwrap()
            .map(|s| s.split(',').map(str::parse::<usize>).collect::<Result<Vec<_>, _>>().unwrap())
            .collect::<Vec<_>>();

        for ((x1, y1), (x2, y2)) in itertools::tuple_windows(&[(p1[0], p1[1]), (p2[0], p2[1])]) {
            if x1 == x2 {
                let (y1, y2) = (y1.min(y2), y1.max(y2));
                for y in y1..=y2 {
                    let index = (x1 + y * 1000) as usize;
                    overlaps += (map[index] == 1) as u32;
                    map[index] += 1;
                }
            } else if y1 == y2 {
                let (x1, x2) = (x1.min(x2), x1.max(x2));
                for x in x1..=x2 {
                    let index = (x + y1 * 1000) as usize;
                    overlaps += (map[index] == 1) as u32;
                    map[index] += 1;
                }
            }
        }
    }

    overlaps.to_string()
}
// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

