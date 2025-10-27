pub fn p1(input: &str) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let line = line.as_bytes();
        let mut i = 1;
        let mut local_len = 0;
        while i < line.len() - 1{
            if line[i] == b'\\' {
                if line[i + 1] == b'x' {
                    local_len += 1;
                    i += 4
                } else {
                    local_len += 1;
                    i += 2;
                }
            } else {
                local_len += 1;
                i += 1;
            }
        }

        res += line.len() as i64 - local_len;
    }

    res
}

pub fn p2(input: &str) -> i64 {
    let mut res: i64 = 0;
    for line in input.lines() {
        if line.is_empty() { continue; }

        let code_len = line.len();
        let mut extra = 2;

        for b in line.bytes() {
            if b == b'"' || b == b'\\' {
                extra += 1;
            }
        }

        res += (extra + code_len) as i64 - code_len as i64;
    }

    res
}
