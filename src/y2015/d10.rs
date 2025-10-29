use std::collections::VecDeque;

fn process(mut q: VecDeque<u8>, times: usize) -> usize {
    for _ in 0..times {
        let start_len = q.len();
        let mut current_item = None;
        let mut cnt = 0;
        for n in 0..start_len {
            let i = q.pop_front().expect("Failed to pop");

            if let Some(ci) = current_item && ci != i {
                current_item = Some(i);
                q.push_back(cnt);
                q.push_back(ci);
                cnt = 1;
            } else {
                current_item = Some(i);
                cnt += 1;
            }

            if n == start_len - 1 {
                q.push_back(cnt);
                q.push_back(i);
            }
        }
    }

    q.len()
}

pub fn p1(input: &str) -> i64 { 
    let mut q = VecDeque::with_capacity(input.len());
    for &ch in input.trim_end().as_bytes() {
        q.push_back(ch - b'0');
    }

    process(q, 40) as i64
}

pub fn p2(input: &str) -> i64 {
    let mut q = VecDeque::with_capacity(input.len());
    for &ch in input.trim_end().as_bytes() {
        q.push_back(ch - b'0');
    }

    process(q, 50) as i64
}

