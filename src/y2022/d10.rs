pub fn p1(input: &str) -> i64 {
    let inter_cycles = [20, 60, 100, 140, 180, 220];
    let mut x_reg = 1i64;
    let mut sched_x = None;
    let mut cycles = 0;
    let mut total_cycles = 0usize;
    let mut result = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        while cycles != 0 {
            total_cycles += 1;
            if inter_cycles.contains(&total_cycles) {
                let tmp = x_reg * total_cycles as i64;
                dbg!(&tmp);
                result += tmp;
            }

            if total_cycles == inter_cycles[5] {
                return result;
            }

            cycles -= 1;
        }

        if let Some(dx) = sched_x.take() {
            x_reg += dx;
        }

        let cmd = line.split_ascii_whitespace().collect::<Vec<_>>();
        match cmd[0] {
            "noop" => cycles += 1,
            "addx" =>  {
                cycles += 2;
                sched_x = Some(cmd[1].parse::<i64>().expect("Failed to parse dx"));
            },
            _ => unreachable!(),
        }
    }

    result
}

pub fn p2(input: &str) -> i64 {
    const SCREEN_WIDTH: usize = 40;
    const SCREEN_HEIGHT: usize = 6;
    let (mut row, mut col) = (0usize, 0usize);
    let mut screen = [[b'.'; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut x_reg = 1i64;
    let mut sched_x = None;
    let mut cycles = 0;
    let mut total_cycles = 0usize;
    for line in input.lines().filter(|l| !l.is_empty()) {
        while cycles != 0 {
            for dx in x_reg-1..x_reg+2 {
                if dx as usize == col {
                    screen[row][col] = b'#';
                }
            }

            total_cycles += 1;
            col += 1;
            if total_cycles % SCREEN_WIDTH == 0 {
                row += 1;
                col = 0;
            }

            cycles -= 1;
        }

        if let Some(dx) = sched_x.take() {
            x_reg += dx;
        }

        let cmd = line.split_ascii_whitespace().collect::<Vec<_>>();
        match cmd[0] {
            "noop" => cycles += 1,
            "addx" =>  {
                cycles += 2;
                sched_x = Some(cmd[1].parse::<i64>().expect("Failed to parse dx"));
            },
            _ => unreachable!(),
        }
    }

    for line in screen {
        println!("{}", str::from_utf8(&line[..]).unwrap());
    }
    0
}
