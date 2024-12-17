use hashbrown::HashMap;

aoc::parts!(1,2);

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;



fn part_1(input: aoc::Input) -> impl ToString {
    const ROW: u32 = 0b11111;
    const COL: u32 = 0b100001000010000100001;

    let (nums, boards) = input.raw().split_once("\n\n").unwrap();

    let mut boards: Vec<(HashMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_ascii_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (board, mark, num) = nums
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .find_map(|n| {
            boards.iter_mut().find_map(|(b, m)| {
                b.get(&n)
                    .map(|i| *m |= 1 << *i)
                    .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                    .map(|_| (b.clone(), *m, n))
            })
        })
        .unwrap();

    let result = board
        .iter() // Iterate over the board without consuming it
        .map(|(n, i)| (mark >> i & 1 ^ 1) * (*n) as u32 * num as u32)
        .sum::<u32>();

    println!("{}", result);

    result // Return the calculated result
}

fn part_2(input: aoc::Input) -> impl ToString {

    let (nums, boards) = input.raw().split_once("\n\n").unwrap();

    let mut boards: Vec<(HashMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_ascii_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let nums = nums.split(',').map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>(); // Specify type here

    let (board, mark, num) = 'outer: loop {
        for n in nums.iter().copied() {
            let mut winning_boards = Vec::new();

            for i in 0..boards.len() {
                let (b, m) = &mut boards[i];
                if let Some(j) = b.get(&n) {
                    *m |= 1 << *j;
                    if (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW) {
                        winning_boards.push(i);
                    }
                }
            }

            if boards.len() == 1 && !winning_boards.is_empty() {
                let (board, mark) = boards.remove(0);
                break 'outer (board, mark, n);
            }

            for i in winning_boards.into_iter().rev() {
                boards.remove(i);
            }
        }
    };

    let result = board
        .iter()
        .map(|(n, i)| (mark >> i & 1 ^ 1) * (*n) as u32 * num as u32)
        .sum::<u32>();

    println!("{}", result);

    result
}