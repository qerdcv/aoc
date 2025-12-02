pub fn p1(input: &str) -> i64 {
    let mut result = 0;
    for range in input.split(",") {
        let (start, to) = range.split_once("-").expect("Invalid range");
        let (start, to) = (
            start.parse::<usize>().unwrap(),
            to.parse::<usize>().unwrap(),
        );

        for n in start..=to {
            let n_str = n.to_string();
            let b = n_str.as_bytes();
            let n_len = n_str.len() / 2;
            if b[n_len..] == b[..n_len] {
                result += n;
            }
        }
    }

    result as i64
}

pub fn p2(input: &str) -> i64 {
    let mut result = 0;
    for range in input.split(",") {
        let (start, to) = range.split_once("-").expect("Invalid range");
        let (start, to) = (
            start.parse::<usize>().unwrap(),
            to.parse::<usize>().unwrap(),
        );

        for n in start..=to {
            let n_str = n.to_string();
            let b: &[u8] = n_str.as_bytes();
            let l = b.len();
            let hl = l / 2;

            'outer: for w in 1..=hl {
                if l % w != 0 {
                    continue;
                }

                let pattern = &b[0..w];
                let mut i = w;
                while i < l {
                    if &b[i..i + w] != pattern {
                        continue 'outer;
                    }
                    i += w;
                }

                result += n as i64;
                break;
            }
        }
    }

    result as i64
}
