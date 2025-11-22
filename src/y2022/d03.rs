use std::collections::HashSet;

pub fn p1(input: &str) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let line = line.as_bytes();
        let mid = line.len() / 2;
        let mut seen: HashSet<&u8> = HashSet::from_iter(line[..mid].into_iter());
        for b in &line[mid..] {
            if seen.contains(b) {
               res += 1 + if b.is_ascii_uppercase() {
                   26 + b - b'A'
               } else {
                   b - b'a'
               } as i64;

                seen.remove(b);
            }
        }
    }

    res
}

pub fn p2(input: &str) -> i64 {
    let mut res = 0;
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    for line in lines.windows(3).step_by(3) {
        let b: HashSet<char> = line[0].chars().collect::<HashSet<_>>()
            .intersection(&line[1].chars().collect::<HashSet<_>>())
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&line[2].chars().collect::<HashSet<_>>())
            .copied()
            .collect();

        for c in b {
            if c.is_ascii_uppercase() {
                res += 27 + (c as u8 - b'A') as i64;
            } else {
                res += 1 + (c as u8 - b'a') as i64;
            }
        }

    }

    res
}
