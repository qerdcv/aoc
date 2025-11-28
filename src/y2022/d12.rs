use std::collections::{HashMap, VecDeque};

fn valid_step(current: u8, next: u8) -> bool {
    current >= next || current.abs_diff(next) < 2
}

pub fn p1(input: &str) -> i64 {
    let mut grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let h = grid.len();
    let w = grid[0].len();

    let (start, end) = (
        grid.iter().flatten().position(|&b| b == b'S').unwrap(),
        grid.iter().flatten().position(|&b| b == b'E').unwrap(),
    );

    let (start, end) = ((start / w, start % w), (end / w, end % w));

    grid[start.0][start.1] = b'a';
    grid[end.0][end.1] = b'z';

    let mut q = VecDeque::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    q.push_back((start.0, start.1, 0usize));
    let mut total_steps = usize::MAX;
    while let Some(i) = q.pop_front() {
        let (row, col, steps) = i;
        if (row, col) == end {
            total_steps = total_steps.min(steps);
            continue;
        }

        if let Some(&s) = visited.get(&(row, col)) && s <= steps {
            continue;
        }

        visited.insert((row, col), steps);

        let ch = grid[row][col];

        if col > 0 && valid_step(ch, grid[row][col - 1]){
            q.push_back((row, col - 1, steps + 1));
        }

        if col < w - 1 && valid_step(ch, grid[row][col + 1]) {
            q.push_back((row, col + 1, steps + 1));
        }

        if row > 0 && valid_step(ch, grid[row - 1][col]) {
            q.push_back((row - 1, col, steps + 1));
        }

        if row < h - 1 && valid_step(ch, grid[row + 1][col]) {
            q.push_back((row + 1, col, steps + 1));
        }
    }

    total_steps as i64
}

pub fn p2(input: &str) -> i64 {
    let mut grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let h = grid.len();
    let w = grid[0].len();

    let (start, end) = (
        grid.iter().flatten().position(|&b| b == b'S').unwrap(),
        grid.iter().flatten().position(|&b| b == b'E').unwrap(),
    );

    let (start, end) = ((start / w, start % w), (end / w, end % w));

    grid[start.0][start.1] = b'a';
    grid[end.0][end.1] = b'z';

    let mut q = VecDeque::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    for row in [0, h - 1] {
        for col in 0..w {
            if grid[row][col] != b'a' {
                continue;
            }

            q.push_back((row, col, 0));
        }
    }

    for col in [0, w - 1] {
        for row in 0..h {
            if grid[row][col] != b'a' {
                continue;
            }

            q.push_back((row, col, 0));
        }
    }

    let mut total_steps = usize::MAX;
    while let Some(i) = q.pop_front() {
        let (row, col, steps) = i;
        if (row, col) == end {
            total_steps = total_steps.min(steps);
            continue;
        }

        if let Some(&s) = visited.get(&(row, col)) && s <= steps {
            continue;
        }

        visited.insert((row, col), steps);

        let ch = grid[row][col];

        if col > 0 && valid_step(ch, grid[row][col - 1]){
            q.push_back((row, col - 1, steps + 1));
        }

        if col < w - 1 && valid_step(ch, grid[row][col + 1]) {
            q.push_back((row, col + 1, steps + 1));
        }

        if row > 0 && valid_step(ch, grid[row - 1][col]) {
            q.push_back((row - 1, col, steps + 1));
        }

        if row < h - 1 && valid_step(ch, grid[row + 1][col]) {
            q.push_back((row + 1, col, steps + 1));
        }
    }

    total_steps as i64
}
