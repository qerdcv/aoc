use std::collections::VecDeque;

const STACK_CNT: usize = 9;

fn parse_stacks(lines: &[&str]) -> [VecDeque<u8>; STACK_CNT] {
    let mut stacks = std::array::from_fn(|_| VecDeque::new());
    for &line in lines {
        for (i, ch) in line.as_bytes().windows(3).step_by(4).enumerate() {
            let ch = ch[1];
            if ch == b' ' {
                continue;
            }

            stacks[i].push_front(ch);
        }
    }

    stacks
}

pub fn p1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut stacks = parse_stacks(&lines[..STACK_CNT-1]);

    for line in &lines[STACK_CNT + 1..] {
        if line.is_empty() {
            continue;
        }

        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let cnt = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;

        let drained = stacks[from].drain(stacks[from].len() - cnt..).rev().collect::<Vec<_>>();
        stacks[to].extend(drained);
    }

    for mut stack in stacks {
        if let Some(ch) = stack.pop_back() {
            print!("{} ", ch as char);
        }
    }

    0
}

pub fn p2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut stacks = parse_stacks(&lines[..STACK_CNT-1]);

    for line in &lines[STACK_CNT + 1..] {
        if line.is_empty() {
            continue;
        }

        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let cnt = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;

        let drained = stacks[from].drain(stacks[from].len() - cnt..).collect::<Vec<_>>();
        stacks[to].extend(drained);
    }

    for mut stack in stacks {
        if let Some(ch) = stack.pop_back() {
            print!("{} ", ch as char);
        }
    }

    0
}
