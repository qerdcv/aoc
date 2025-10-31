fn is_good(input: &[u8]) -> bool {
    let n = input.len();
    let mut double_cnt = 0;
    let mut prev_double_idx = 0;
    let mut three_seq = false;
    for (i, &b) in input.iter().enumerate() {
        if matches!(b, b'i' | b'o' | b'l') {
            return false;
        }

        if i + 2 < n && input[i] == input[i + 1] - 1 && input[i + 1] == input[i + 2] - 1 {
            three_seq = true;
        }

        if i + 1 < n && i != prev_double_idx && input[i] == input[i + 1] {
            prev_double_idx = i + 1;
            double_cnt += 1;
        }
    }

    double_cnt >= 2 && three_seq
}

fn process(input: &str) -> String {
    let mut input: Vec<u8> = input.trim_end().as_bytes().to_vec();
    let n = input.len() - 1;
    loop {
        let mut remainder = 1;
        let mut i = n;
        while remainder != 0 {
            let ch = &mut input[i];
            *ch += remainder;
            remainder = 0;
            if matches!(ch, b'i' | b'o' | b'l') {
                *ch += 1;
            }

            if *ch > b'z' {
                remainder = 1;
                i -= 1;
                *ch = b'a' + (*ch - b'z' - 1);
            }
        }

        if is_good(&input) {
            break;
        }
    }

    String::from_utf8(input).expect("Failed to parse string")
}

pub fn p1(input: &str) -> i64 {
    println!("{:?}", process(input));
    0
}

pub fn p2(_: &str) -> i64 {
    println!("{:?}", process("hxbxxyzz" /* first part output */));

    0
}
