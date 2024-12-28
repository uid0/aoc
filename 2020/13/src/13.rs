aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let lines: Vec<&str> = input.as_lines().into_iter().copied().collect(); // Corrected line
    let earliest_departure: i32 = lines[0].parse().unwrap();
    let bus_ids: Vec<i32> = lines[1]
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    let (best_bus_id, waiting_time) = bus_ids
        .iter()
        .map(|&bus_id| {
            let next_departure = ((earliest_departure + bus_id - 1) / bus_id) * bus_id;
            (bus_id, next_departure - earliest_departure)
        })
        .min_by_key(|&(_, wait_time)| wait_time)
        .unwrap();

    (best_bus_id * waiting_time).to_string()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let lines: Vec<&str> = input.as_lines().into_iter().copied().collect();
    let bus_ids: Vec<(usize, i64)> = lines[1]
        .split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect();

    let mut time = 0;
    let mut step = 1;

    for &(offset, bus_id) in &bus_ids {
        while (time + offset as i64) % bus_id != 0 {
            time += step;
        }
        step *= bus_id;
    }

    time.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2_example_1() {
        let input = "0\n7,13,x,x,59,x,31,19";
        let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
        let bus_ids: Vec<(usize, i64)> = lines[1]
            .split(',')
            .enumerate()
            .filter(|&(_, s)| s != "x")
            .map(|(i, s)| (i, s.parse().unwrap()))
            .collect();

        let mut time = 0;
        let mut step = 1;

        for &(offset, bus_id) in &bus_ids {
            while (time + offset as i64) % bus_id != 0 {
                time += step;
            }
            step *= bus_id;
        }

        assert_eq!(time, 1068781);
    }
    #[test]
    fn test_part_2_example_2() {
        let input = "0\n17,x,13,19";
        let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
        let bus_ids: Vec<(usize, i64)> = lines[1]
            .split(',')
            .enumerate()
            .filter(|&(_, s)| s != "x")
            .map(|(i, s)| (i, s.parse().unwrap()))
            .collect();

        let mut time = 0;
        let mut step = 1;

        for &(offset, bus_id) in &bus_ids {
            while (time + offset as i64) % bus_id != 0 {
                time += step;
            }
            step *= bus_id;
        }
        assert_eq!(time, 3417);
    }
    #[test]
    fn test_part_2_example_3() {
        let input = "0\n67,7,59,61";
        let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
        let bus_ids: Vec<(usize, i64)> = lines[1]
            .split(',')
            .enumerate()
            .filter(|&(_, s)| s != "x")
            .map(|(i, s)| (i, s.parse().unwrap()))
            .collect();

        let mut time = 0;
        let mut step = 1;

        for &(offset, bus_id) in &bus_ids {
            while (time + offset as i64) % bus_id != 0 {
                time += step;
            }
            step *= bus_id;
        }
        assert_eq!(time, 754018);
    }
    #[test]
    fn test_part_2_example_4() {
        let input = "0\n67,x,7,59,61";
        let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
        let bus_ids: Vec<(usize, i64)> = lines[1]
            .split(',')
            .enumerate()
            .filter(|&(_, s)| s != "x")
            .map(|(i, s)| (i, s.parse().unwrap()))
            .collect();

        let mut time = 0;
        let mut step = 1;

        for &(offset, bus_id) in &bus_ids {
            while (time + offset as i64) % bus_id != 0 {
                time += step;
            }
            step *= bus_id;
        }
        assert_eq!(time, 779210);
    }
    #[test]
    fn test_part_2_example_5() {
        let input = "0\n67,7,x,59,61";
        let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
        let bus_ids: Vec<(usize, i64)> = lines[1]
            .split(',')
            .enumerate()
            .filter(|&(_, s)| s != "x")
            .map(|(i, s)| (i, s.parse().unwrap()))
            .collect();

        let mut time = 0;
        let mut step = 1;

        for &(offset, bus_id) in &bus_ids {
            while (time + offset as i64) % bus_id != 0 {
                time += step;
            }
            step *= bus_id;
        }
        assert_eq!(time, 1261476);
    }
    #[test]
    fn test_part_2_example_6() {
        let input = "0\n1789,37,47,1889";
        let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
        let bus_ids: Vec<(usize, i64)> = lines[1]
            .split(',')
            .enumerate()
            .filter(|&(_, s)| s != "x")
            .map(|(i, s)| (i, s.parse().unwrap()))
            .collect();

        let mut time = 0;
        let mut step = 1;

        for &(offset, bus_id) in &bus_ids {
            while (time + offset as i64) % bus_id != 0 {
                time += step;
            }
            step *= bus_id;
        }
        assert_eq!(time, 1202161486);
    }

    #[test]
    fn test_part_1() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let lines: Vec<&str> = input.lines().map(|x| x.trim()).collect();
        let earliest_departure: i32 = lines[0].parse().unwrap();
        let bus_ids: Vec<i32> = lines[1]
            .split(',')
            .filter(|&s| s != "x")
            .map(|s| s.parse().unwrap())
            .collect();

        let (best_bus_id, waiting_time) = bus_ids
            .iter()
            .map(|&bus_id| {
                let next_departure = ((earliest_departure + bus_id - 1) / bus_id) * bus_id;
                (bus_id, next_departure - earliest_departure)
            })
            .min_by_key(|&(_, wait_time)| wait_time)
            .unwrap();

        assert_eq!(best_bus_id * waiting_time, 295);
    }
}
