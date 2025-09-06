fn is_nice(input: &[u8]) -> bool {
    let mut vovels = 0;
    let mut has_double_letter = false;
    for w in input.windows(2) {
        let c = w[0];
        let next_ch = w[1];

        if b"aeiou".contains(&c) {
            vovels += 1;
        }

        if c == next_ch {
            has_double_letter = true;
        }

        if matches!(
            (c, next_ch),
            (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y')
        ) {
            return false;
        }
    }

    if let Some(last) = input.last() {
        if b"aeiou".contains(last) {
            vovels += 1;
        }
    }

    vovels >= 3 && has_double_letter
}

pub fn p1(input: &[u8]) -> i64 {
    input
        .split(|&b| b == b'\n')
        .filter(|line| is_nice(line))
        .count() as i64
}

fn is_nice2(input: &[u8]) -> bool {
    let mut pair_indices: std::collections::HashMap<(u8, u8), usize> =
        std::collections::HashMap::new();
    let mut has_pair_repeat = false;

    for (i, w) in input.windows(2).enumerate() {
        let pair = (w[0], w[1]);
        if let Some(&prev_idx) = pair_indices.get(&pair) {
            if i > prev_idx + 1 {
                has_pair_repeat = true;
                break;
            }
        } else {
            pair_indices.insert(pair, i);
        }
    }

    if !has_pair_repeat {
        return false;
    }

    let has_repeat_with_gap = input.windows(3).any(|w| w[0] == w[2]);

    has_repeat_with_gap
}

pub fn p2(input: &[u8]) -> i64 {
    input
        .split(|&b| b == b'\n')
        .filter(|&line| is_nice2(line))
        .count() as i64
}
