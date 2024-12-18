use std::collections::VecDeque;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    // According to the problem:
    // 1. We have a memory space of size 71x71 (coordinates 0 through 70 inclusive in both directions).
    // 2. We read the first 1024 lines of input, each containing X,Y coordinates where bytes fall and corrupt that location.
    // 3. After placing these 1024 corrupted bytes, we need to find the shortest path from (0,0) to (70,70).
    //
    // We will:
    // - Parse the first 1024 coordinates from the input.
    // - Mark those coordinates as corrupted in a 71x71 boolean grid.
    // - Then use a shortest path search (BFS) to find the minimum steps.
    //
    // If no path is found, we might return something like 0, but presumably a path should exist.
    //
    // The user wants tests that can validate this logic. We'll write a helper function that can be tested.

    let coords = input
        .lines()
        .take(1024)
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();

    let steps = shortest_path_after_corruption(71, 71, &coords, (0, 0), (70, 70));
    steps
}

// This function performs the simulation and BFS to find the shortest path.
fn shortest_path_after_corruption(
    width: usize,
    height: usize,
    corrupted_coords: &[(usize, usize)],
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    let mut grid = vec![vec![false; height]; width];

    // Mark corrupted cells
    for &(x, y) in corrupted_coords {
        if x < width && y < height {
            grid[x][y] = true;
        }
    }

    // BFS
    let mut dist = vec![vec![usize::MAX; height]; width];
    dist[start.0][start.1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((cx, cy)) = queue.pop_front() {
        if (cx, cy) == goal {
            return dist[cx][cy];
        }
        let current_dist = dist[cx][cy];
        for &(dx, dy) in &directions {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                let (nx, ny) = (nx as usize, ny as usize);
                if !grid[nx][ny] && dist[nx][ny] == usize::MAX {
                    dist[nx][ny] = current_dist + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // If no path found
    usize::MAX
}

fn part_2(input: aoc::Input) -> impl ToString {
    // For part 2:
    // We need to determine the FIRST byte that, when it falls, will block the path from (0,0) to (70,70).
    // We will simulate the falling bytes one by one:
    //  1. Add a byte to the corruption grid.
    //  2. Check if there is still a path from start to goal.
    //  3. The moment we detect there's no path, we return the coordinate that caused it.
    //
    // Steps:
    // - We'll parse all input coordinates.
    // - We'll have a mutable grid and add corruption one by one.
    // - After each addition, we'll run a BFS to check connectivity.
    // - Once not reachable, return that coordinate.

    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();

    let (x, y) = first_blocking_coordinate(71, 71, &coords, (0, 0), (70, 70));
    format!("{},{}", x, y)
}

// Incrementally add corrupted cells and test for path availability
fn first_blocking_coordinate(
    width: usize,
    height: usize,
    coords: &[(usize, usize)],
    start: (usize, usize),
    goal: (usize, usize),
) -> (usize, usize) {
    let mut grid = vec![vec![false; height]; width];

    for &(x, y) in coords {
        // Add corruption
        if x < width && y < height {
            grid[x][y] = true;
        }

        // Check path
        if !is_path_available(&grid, start, goal) {
            return (x, y);
        }
    }
    // If no blocking coordinate found, just return last one or something meaningful
    // But the problem states we should find the first that blocks, so presumably we always return.
    (0, 0)
}

fn is_path_available(grid: &[Vec<bool>], start: (usize, usize), goal: (usize, usize)) -> bool {
    let width = grid.len();
    let height = grid[0].len();

    if grid[start.0][start.1] || grid[goal.0][goal.1] {
        return false;
    }

    let mut dist = vec![vec![usize::MAX; height]; width];
    dist[start.0][start.1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some((cx, cy)) = queue.pop_front() {
        if (cx, cy) == goal {
            return true;
        }
        let current_dist = dist[cx][cy];
        for &(dx, dy) in &directions {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                let (nx, ny) = (nx as usize, ny as usize);
                if !grid[nx][ny] && dist[nx][ny] == usize::MAX {
                    dist[nx][ny] = current_dist + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    false
}
