pub fn p1(input: &str) -> i64 {
    let mut score = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let line = line.as_bytes();
        let a = line[0] - b'A';
        let b = line[2] - b'X';
    
        score += b as i64 + 1;
        if a == b {
            score += 3;
        } else if a == 0 && b == 1 || a == 1 && b == 2 || a == 2 && b == 0 {
            score += 6;
        } else {
        }
    }

    score
}

pub fn p2(input: &str) -> i64 {
    // rock == 0 (A, X)
    // paper == 1 (B, Y)
    // scissors == 2 (C, Z)
    let mut score = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let line = line.as_bytes();
        let a = line[0] - b'A';
        let b = line[2] - b'X';
        match b {
            0 => { // lose
                score += match a {
                    0 => 3,
                    1 => 1,
                    2 => 2,
                    _ => unreachable!(),
                };
            },
            1 => { // draw
                score +=  4 + a as i64;
            },
            2 => { // win
                score += 6  + match a {
                    0 => 2,
                    1 => 3,
                    2 => 1,
                    _ => unreachable!(),
                }
            },
            _ => unreachable!("b {}", b as char),
        }
    }

    score
}
