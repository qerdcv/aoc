
#[inline(always)]
fn translate(u: u8) -> usize {
    (u - b'a') as usize
}

pub fn p1(input: &str) -> i64 {
    let input = input.as_bytes().to_vec();
    let n = input.len();
    let mut seen = vec![0u8; 26];
    let (mut start, mut end) = (0, 0);
    loop {
        let current = translate(input[end]);
        if seen[current] == 0 {
            seen[current] += 1;
            end += 1;
        } else if seen[current] == 1 {
            seen[translate(input[start])] -= 1;
            start += 1;
        }

        if end - start == 4 {
            return end as i64;
        }

        if end == n {
            return -1;
        }
    }
}

pub fn p2(input: &str) -> i64 {
    let input = input.as_bytes().to_vec();
    let n = input.len();
    let mut seen = vec![0u8; 26];
    let (mut start, mut end) = (0, 0);
    loop {
        let current = translate(input[end]);
        if seen[current] == 0 {
            seen[current] += 1;
            end += 1;
        } else if seen[current] == 1 {
            seen[translate(input[start])] -= 1;
            start += 1;
        }

        if end - start == 14 {
            return end as i64;
        }

        if end == n {
            return -1;
        }
    }

}
