use std::io::{BufRead, BufReader};

pub fn p1(input: &[u8]) -> i64 {
    let mut result = 0;
    let mut reader = BufReader::new(input);
    let mut buffer = Vec::new();
    while reader.read_until(b'\n', &mut buffer).unwrap() != 0 {
        buffer.pop(); // drop the \n byte
        let numbers: Vec<i64> = buffer
            .split(|&b| b == b'x')
            .map(|p| std::str::from_utf8(p).unwrap().parse::<i64>().unwrap())
            .collect(); // l | w | h

        let lw = numbers[0] * numbers[1];
        let wh = numbers[1] * numbers[2];
        let hl = numbers[2] * numbers[0];
        result += (2 * lw) + (2 * wh) + (2 * hl) + lw.min(wh).min(hl);

        buffer.clear();
    }

    result
}

pub fn p2(input: &[u8]) -> i64 {
    let mut result = 0;
    let mut reader = BufReader::new(input);
    let mut buffer = Vec::new();
    while reader.read_until(b'\n', &mut buffer).unwrap() != 0 {
        buffer.pop(); // drop the \n byte
        let mut numbers: Vec<i64> = buffer
            .split(|&b| b == b'x')
            .map(|p| std::str::from_utf8(p).unwrap().parse::<i64>().unwrap())
            .collect(); // l | w | h

        numbers.sort();
        result += (numbers[0] + numbers[0] + numbers[1] + numbers[1])
            + (numbers[0] * numbers[1] * numbers[2]);
        buffer.clear();
    }

    result
}
