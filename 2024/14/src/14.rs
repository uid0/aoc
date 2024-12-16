use std::collections::HashMap;

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut robots: Vec<(i32, i32, i32, i32)> = input
        .raw()
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let (p, v) = line.split_once(" v=").unwrap();
            let (px, py) = p.split_once(',').unwrap();
            let (vx, vy) = v.split_once(',').unwrap();
            Some((
                px[2..].parse::<i32>().unwrap(),
                py[2..].parse::<i32>().unwrap(),
                vx[2..].parse::<i32>().unwrap(),
                vy[2..].parse::<i32>().unwrap(),
            ))
        })
        .collect();

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    for _ in 0..100 {
        for (_i, (px, py, vx, vy)) in robots.iter_mut().enumerate() {
            *px = (*px + *vx + 101) % 101;
            *py = (*py + *vy + 103) % 103;
            *grid.entry((*px, *py)).or_insert(0) += 1;
        }
    }

    let mut quadrants = [0; 4];
    for ((x, y), count) in grid.iter() {
        if *x < 50 && *y < 51 {
            quadrants[0] += count;
        } else if *x >= 50 && *y < 51 {
            quadrants[1] += count;
        } else if *x < 50 && *y >= 51 {
            quadrants[2] += count;
        } else if *x >= 50 && *y >= 51 {
            quadrants[3] += count;
        }
    }

    quadrants.iter().product::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part_1(input.into()), "12");
    }
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
