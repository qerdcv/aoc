use std::i64;

fn max_cap(batts: &[i64], k: usize) -> i64 {
    let n = batts.len();
    let mut items = Vec::with_capacity(k);
    let mut start = 0;

    for to_find in (0..k).rev() {
        let end = n - to_find - 1;

        let mut max_i = start;
        let mut max_c = batts[start];

        for i in start..=end {
            if batts[i] > max_c {
                max_c = batts[i];
                max_i = i;
            }
        }

        items.push(max_c);
        start = max_i + 1;
    }

    let mut cap = 0;
    for c in items {
        cap = cap * 10 + c;
    }

    cap
}

pub fn p1(input: &str) -> i64 {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| max_cap(&l.as_bytes().to_vec().iter().map(|&b| (b - b'0') as i64).collect::<Vec<_>>(), 2))
        .sum()
}

pub fn p2(input: &str) -> i64 {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| max_cap(&l.as_bytes().to_vec().iter().map(|&b| (b - b'0') as i64).collect::<Vec<_>>(), 12))
        .sum()
}
