pub fn p1(input: &str) -> i64 {
    let grid = input
        .lines()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let h = grid.len();
    let w = grid[0].len();

    let mut res = 0;
    for row in 0..h {
        for col in 0..w {
            let t = grid[row][col];

            let l = (0..col).all(|x| grid[row][x] < t);
            let r = (col + 1..w).all(|x| grid[row][x] < t);
            let u = (0..row).all(|y| grid[y][col] < t);
            let b = (row + 1..h).all(|y| grid[y][col] < t);

            if l || r || u || b {
                res += 1;
            }
        }
    }

    res as i64
}

pub fn p2(input: &str) -> i64 {
    let grid = input
        .lines()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let h = grid.len();
    let w = grid[0].len();

    let mut res = 0;
    for row in 0..h {
        for col in 0..w {
            let t = grid[row][col];

            let mut l = 0;
            for x in (0..col).rev() {
                l += 1;
                if grid[row][x] >= t {
                    break;
                }
            }


            let mut r = 0;
            for x in col + 1..w {
                r += 1;
                if grid[row][x] >= t {
                    break;
                }
            }

            let mut u = 0;
            for y in (0..row).rev() {
                u += 1;
                if grid[y][col] >= t {
                    break;
                }
            }

            let mut b = 0;
            for y in row + 1..h {
                b += 1;
                if grid[y][col] >= t {
                    break;
                }
            }

            res = res.max(l * r * u * b);
        }
    }

    res as i64
}
