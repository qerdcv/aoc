pub fn p1(input: &[u8]) -> i64 {
    input
        .iter()
        .map(|&c| {
            if c == b'(' {
                1
            } else if c == b')' {
                -1
            } else {
                0
            }
        })
        .sum()
}

pub fn p2(input: &[u8]) -> i64 {
    let mut floor = 0;
    for (idx, &i) in input.iter().enumerate() {
        if i == b'(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor < 0 {
            return (idx + 1) as i64;
        }
    }

    floor
}
