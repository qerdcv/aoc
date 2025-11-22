pub fn p1(input: &str) -> i64 {
    let mut result = 0;
    let mut parsing = false;
    let mut sign = 1;
    let mut local_num = 0;
    for &b in input.as_bytes() {
        if b == b'-' || b >= b'0' && b <= b'9' {
            parsing = true;
            if b == b'-' {
                sign = -1;
                continue;
            }

            local_num = (local_num * 10) + (b - b'0') as i64;
        } else if parsing {
            result += local_num * sign;
            sign = 1;
            parsing = false;
            local_num = 0;
        }
    }

    result
}

pub fn p2(input: &str) -> i64 {
    let mut result = 0;
    for &b in input.as_bytes() {
    }

    result
}
