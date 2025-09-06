use std::collections::HashSet;

pub fn p1(input: &[u8]) -> i64 {
    let mut seen = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    seen.insert((x, y));

    for &c in input {
        match c {
            b'>' => x += 1,
            b'<' => x -= 1,
            b'^' => y -= 1,
            b'v' => y += 1,
            _ => continue,
        }

        seen.insert((x, y));
    }

    seen.len() as i64
}

pub fn p2(input: &[u8]) -> i64 {
    let mut seen = HashSet::new();
    let mut santa = (0, 0);
    let mut robo_santa = (0, 0);
    seen.insert(santa);
    for (i, &c) in input.iter().enumerate() {
        let current = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };

        match c {
            b'>' => current.0 += 1,
            b'<' => current.0 -= 1,
            b'^' => current.1 -= 1,
            b'v' => current.1 += 1,
            _ => continue,
        }

        seen.insert(*current);
    }

    seen.len() as i64
}
