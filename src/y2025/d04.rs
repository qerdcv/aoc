pub fn p1(input: &str) -> i64 {
    let grid: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let w = grid[0].len();
    let h = grid.len();

    let mut result = 0;
    for row in 0..h {
        for col in 0..w {
            if grid[row][col] == b'.' {
                continue;
            }


            let mut cnt = 0;
            for y in row.saturating_sub(1)..=(row + 1).min(h - 1) {
                for x in col.saturating_sub(1)..=(col + 1).min(w - 1) {
                    if (y, x) == (row, col) {
                        continue;
                    }

                    if grid[y][x] == b'@' {
                        cnt += 1;
                    }
                }
            }

            if cnt < 4 {
                result += 1;
            }
        }
    }

    for row in grid {
        println!("{}", str::from_utf8(&row).unwrap());
    }

    result
}

pub fn p2(input: &str) -> i64 {
    let mut grid: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let w = grid[0].len();
    let h = grid.len();

    let mut result = 0;
    let mut to_remove = Vec::new();
    loop {
        to_remove.clear();
        for row in 0..h {
            for col in 0..w {
                if grid[row][col] == b'.' {
                    continue;
                }


                let mut cnt = 0;
                for y in row.saturating_sub(1)..=(row + 1).min(h - 1) {
                    for x in col.saturating_sub(1)..=(col + 1).min(w - 1) {
                        if (y, x) == (row, col) {
                            continue;
                        }

                        if grid[y][x] == b'@' {
                            cnt += 1;
                        }
                    }
                }

                if cnt < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        } else {
            result += to_remove.len() as i64;
            for &(row, col) in &to_remove {
                grid[row][col] = b'.';
            }
        }
    }

    for row in grid {
        println!("{}", str::from_utf8(&row).unwrap());
    }

    result
}
