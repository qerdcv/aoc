pub fn p1(input: &str) -> i64 {
    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.as_slice() {
            [from, "to", to, "=", dist] => {
                let dist = dist.parse::<i64>().expect("failed to parse distance");

            },
            _ => panic!("invalid line: {}", line),
        }
    }

    0
}

pub fn p2(input: &str) -> i64 { 0 }
