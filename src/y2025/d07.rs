pub fn p1(input: &str) -> i64 {
    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let h = grid.len();
    let w = grid[0].len();
    let mut splits = 0;

    let mut beams = vec![false; w];
    beams[w / 2] = true;
    for y in 1..h-1 {
        let next_y = y + 1;
        for x in 0..w {
            if !beams[x] {
                continue;
            }

            if grid[next_y][x] == b'^' {
                splits += 1;
                beams[x] = false;
                beams[x + 1] = true;
                beams[x - 1] = true;
            }
        }
    }

    splits
}

pub fn p2(input: &str) -> i64 {
    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let h = grid.len();
    let w = grid[0].len();

    let mut beams = vec![0; w];
    beams[w / 2] = 1;
    for y in 1..h {
        let next_y = y + 1;
        if next_y == h {
            break;
        }

        let mut next_beams = vec![0; w];
        for x in 0..w {
            let count = beams[x];
            if count == 0 {
                continue;
            }

            if grid[next_y][x] == b'^' {
                next_beams[x - 1] += count;
                next_beams[x + 1] += count;
            } else {
                next_beams[x] += count;
            }
        }

        beams = next_beams;
    }

    beams.iter().sum()
}
