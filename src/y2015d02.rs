pub fn p1(input: &str) -> i64 {
    let mut result = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let numbers: Vec<i64> = line
            .split('x')
            .map(|p| p.parse::<i64>().expect("failed to parse number"))
            .collect(); // l | w | h

        let lw = numbers[0] * numbers[1];
        let wh = numbers[1] * numbers[2];
        let hl = numbers[2] * numbers[0];
        result += (2 * lw) + (2 * wh) + (2 * hl) + lw.min(wh).min(hl);
    }

    result
}

pub fn p2(input: &str) -> i64 {
    let mut result = 0;
    for line in input.lines() {
        let mut numbers: Vec<i64> = line
            .split('x')
            .map(|p| p.parse::<i64>().expect("number expected"))
            .collect(); // l | w | h

        numbers.sort();
        result += (numbers[0] + numbers[0] + numbers[1] + numbers[1])
            + (numbers[0] * numbers[1] * numbers[2]);
    }

    result
}
