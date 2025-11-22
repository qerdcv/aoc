pub fn p1(input: &str) -> i64 {
    let mut res = 0;
    let mut elf_total = 0;

    for line in input.lines() {
        if line.is_empty() {
            res = res.max(elf_total);
            elf_total = 0;
            continue;
        }

        elf_total += line.parse::<i64>().expect("Failed to parse int");
    }

    res
}

pub fn p2(input: &str) -> i64 {
    let mut res: [i64; 3] = [0i64; 3];
    let mut elf_total = 0;

    for line in input.lines() {
        if line.is_empty() {
            if res[0] < elf_total {
                res[2] = res[1];
                res[1] = res[0];
                res[0] = elf_total;
            } else if res[1] < elf_total {
                res[2] = res[1];
                res[1] = elf_total;
            } else if res[2] < elf_total {
                res[2] = elf_total;
            }

            elf_total = 0;
            continue;
        }

        elf_total += line.parse::<i64>().expect("Failed to parse int");
    }

    res.iter().sum()
}
