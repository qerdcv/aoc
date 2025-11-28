use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Imm(usize),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        if value == "old" {
            Self::Old
        } else {
            Self::Imm(value.parse::<usize>().expect("Invalid operation"))
        }
    }
}

impl Operand {
    fn into_usize(&self, old: usize) -> usize {
        match self {
            Operand::Old => old,
            Operand::Imm(val) => val.clone(),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl From<Vec<&str>> for Operation {
    fn from(value: Vec<&str>) -> Self {
        assert!(value.len() == 3, "Invalid operation len");
        let (op1, op2) = (Operand::from(value[0]), Operand::from(value[2]));
        match value[1] {
            "*" => Self::Mul(op1, op2),
            "+" => Self::Add(op1, op2),
            _ => panic!("invalid operation: {}", value[1]),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    div_by: usize,
    true_cond: usize,
    false_cond: usize,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines();
    loop {
        if lines.next().is_none() {
            break;
        }

        let items = lines
            .next()
            .expect("Starting items expected")
            .split(": ")
            .last()
            .expect("No starting items")
            .split(", ")
            .map(|i| i.parse::<usize>().expect("Failed to parse starting item"))
            .collect::<VecDeque<_>>();
        let operation = Operation::from(lines
            .next()
            .expect("No operation")
            .split(" = ")
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<_>>());
        let div_by = lines
            .next()
            .expect("Div cond expected")
            .split_ascii_whitespace()
            .last()
            .expect("failed to get div cond")
            .parse::<usize>()
            .expect("failed to parse divider");
        let true_cond = lines
            .next()
            .expect("Failed to get true cond")
            .split_ascii_whitespace()
            .last()
            .expect("No true cond")
            .parse::<usize>()
            .expect("Invalid true cond");
        let false_cond = lines
            .next()
            .expect("Failed to get false cond")
            .split_ascii_whitespace()
            .last()
            .expect("No true cond")
            .parse::<usize>()
            .expect("Invalid false cond");

        lines.next();
        monkeys.push(Monkey { items, operation, div_by, true_cond, false_cond })
    }

    monkeys
}

pub fn p1(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);
    let mut insp_cnt = vec![0usize; monkeys.len()];
    let n = monkeys.len();
    for _ in 0..20 {
        for i in 0..n {
            let (operation, div_by, true_cond, false_cond) = {
                let m = &monkeys[i];
                (m.operation.clone(), m.div_by, m.true_cond, m.false_cond)
            };

            while let Some(old) = {
                let m = &mut monkeys[i];
                m.items.pop_front()
            } {
                insp_cnt[i] += 1;
                let new = match &operation {
                    Operation::Add(op1, op2) => op1.into_usize(old) + op2.into_usize(old),
                    Operation::Mul(op1, op2) => op1.into_usize(old) * op2.into_usize(old),
                } / 3;

                let to = if new % div_by == 0 { true_cond } else { false_cond };
                // println!("{}. {} {:?} = {} -> {}", i, old, operation, new, to);
                monkeys[to].items.push_back(new);
            }
        }
    }
    insp_cnt.sort_by(|a, b| b.cmp(a));

    (insp_cnt[0] * insp_cnt[1]) as i64
}

pub fn p2(input: &str) -> i64 {
    0
}
