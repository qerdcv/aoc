pub fn p1(input: &str) -> i64 {
    let mut numbers = Vec::new();
    let mut operations = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let parsed_line = line.split_whitespace().collect::<Vec<_>>();
        if parsed_line[0].as_bytes()[0].is_ascii_digit() {
            numbers.push(parsed_line.iter().map(|i| i.parse::<i64>().unwrap()).collect::<Vec<_>>());
        } else {
            operations.extend(parsed_line);
        }
    }

    operations.iter().enumerate().map(|(i, &op)| {
        let m = numbers.iter().map(|n| n[i]);
        match op {
            "*" => m.fold(1, |a, x| a * x),
            "+" => m.sum(),
            _ => unreachable!(),
        }
    }).sum()
}

pub fn p2(input: &str) -> i64 {
    let mut numbers = Vec::new();
    let mut operations = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let bytes = line.as_bytes();
        if bytes[0].is_ascii_digit() || bytes[0] == b' ' {
            numbers.push(bytes.to_vec());
        } else {
            operations.extend(line.split_whitespace());
        }
    }

    let mut columned = Vec::new();
    let w = numbers[0].len();
    let h = numbers.len();
    let mut start = 0;
    for col in 0..w {
        if col != w - 1 && !numbers.iter().all(|l| l[col] == b' ') {
            continue;
        }

        let col = if col == w - 1 { col + 1 } else { col };

        let mut new_numbers = Vec::new();
        for x in (start..col).rev() {
            let mut number = 0;
            for y in 0..h {
                if numbers[y][x] == b' ' {
                    continue;
                }

                number = number * 10 + (numbers[y][x] - b'0') as i64;
            }
            new_numbers.push(number);
        }

        columned.push(new_numbers);
        start = col + 1;
    }

    operations.iter().enumerate().map(|(i, &op)| {
        match op {
            "*" => columned[i].iter().fold(1, |a, i| a * i),
            "+" => columned[i].iter().sum(),
            _ => unreachable!(),
        }
    }).sum()
}
