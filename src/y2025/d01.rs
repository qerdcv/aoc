const DIAL_POSITIONS: i64 = 100;

pub fn p1(input: &str) -> i64 {
    let mut dial = 50;
    let mut result = 0;
    for (dir, times) in input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let l = l.as_bytes();
            (l[0], str::from_utf8(&l[1..]).unwrap().parse::<i64>().unwrap())
        }) {
        dial = match dir {
            b'R' => dial + times,
            b'L' => dial - times,
            _ => panic!("Unexpected direction"),
        }.rem_euclid(DIAL_POSITIONS);

        if dial == 0{
            result += 1;
        }
    }

    result
}

pub fn p2(input: &str) -> i64 {
    let mut dial = 50;
    let mut result = 0;
    for (dir, times) in input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let l = l.as_bytes();
            (l[0], str::from_utf8(&l[1..]).unwrap().parse::<i64>().unwrap())
        }) {
        result += match dir {
            b'R' => {
                let h = if dial == 0 { DIAL_POSITIONS } else { DIAL_POSITIONS - dial };
                if times < h {
                    0
                } else {
                    1 + (times - h) / DIAL_POSITIONS
                }
            }
            b'L' => {
                let h = if dial == 0 { DIAL_POSITIONS } else { dial };
                if times < h {
                    0
                } else {
                    1 + (times - h) / DIAL_POSITIONS
                }
            }
            _ => panic!("Unexpected direction"),
        };

        dial = match dir {
            b'R' => dial + times,
            b'L' => dial - times,
            _ => panic!("Unexpected direction"),
        }.rem_euclid(DIAL_POSITIONS);
    }

    result
}
