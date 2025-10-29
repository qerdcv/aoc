use md5::{Digest, Md5};

pub fn p1(input: &str) -> i64 {
    let input = input.trim_end();
    let mut i = 0;
    let mut hasher = Md5::new();
    loop {
        hasher.update(input);
        hasher.update(i.to_string().as_bytes());
        let hash = hasher.finalize_reset();
        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 {
            return i;
        }

        i += 1;
    }
}

pub fn p2(input: &str) -> i64 {
    let input = input.trim_end();
    let mut hasher = Md5::new();
    let mut i = 0;
    loop {
        hasher.update(input);
        hasher.update(&i.to_string());
        let hash = hasher.finalize_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;
        }

        i += 1;
    }
}
