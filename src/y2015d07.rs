use std::collections::HashMap;

#[derive(Clone)]
enum Operand {
    Imm(u16),
    Reg(String),
}
#[derive(Clone)]
enum Expr {
    Assign(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, u16),
    RShift(Operand, u16),
}

fn parse_operand(s: &str) -> Operand {
    if let Ok(n) = s.parse::<u16>() {
        Operand::Imm(n)
    } else {
        Operand::Reg(s.to_string())
    }
}

fn parse_line(line: &str) -> (String, Expr) {
    let toks: Vec<&str> = line.split_ascii_whitespace().collect();
    match toks.as_slice() {
        // 123 -> x   |   lf -> x
        [src, "->", dst] => {
            (dst.to_string(), Expr::Assign(parse_operand(src)))
        }
        // NOT x -> y
        ["NOT", x, "->", dst] => {
            (dst.to_string(), Expr::Not(parse_operand(x)))
        }
        // x AND y -> z  |  x OR y -> z  |  x LSHIFT 2 -> z  |  x RSHIFT 2 -> z
        [a, op, b, "->", dst] => {
            let (a, b) = (parse_operand(a), parse_operand(b));
            let expr = match *op {
                "AND"    => Expr::And(a, b),
                "OR"     => Expr::Or(a, b),
                "LSHIFT" => Expr::LShift(a, match b { Operand::Imm(n) => n, _ => panic!("LSHIFT by non-imm") }),
                "RSHIFT" => Expr::RShift(a, match b { Operand::Imm(n) => n, _ => panic!("RSHIFT by non-imm") }),
                _ => panic!("unknown op {op}"),
            };
            (dst.to_string(), expr)
        }
        _ => panic!("bad line: {line}"),
    }
}

fn val(reg: &str, circ: &HashMap<String, Expr>, memo: &mut HashMap<String, u16>) -> u16 {
    if let Some(&v) = memo.get(reg) { return v; }
    let expr = circ.get(reg).unwrap_or_else(|| panic!("unknown wire {reg}"));
    let eval_op = |op: &Operand, circ: &HashMap<String, Expr>, memo: &mut HashMap<String, u16>| -> u16 {
        match op {
            Operand::Imm(n)    => *n,
            Operand::Reg(r)    => val(r, circ, memo),
        }
    };

    let out = match expr {
        Expr::Assign(a)       => eval_op(a, circ, memo),
        Expr::Not(a)          => !eval_op(a, circ, memo),
        Expr::And(a, b)       => eval_op(a, circ, memo) &  eval_op(b, circ, memo),
        Expr::Or(a, b)        => eval_op(a, circ, memo) |  eval_op(b, circ, memo),
        Expr::LShift(a, n)    => eval_op(a, circ, memo) << n,
        Expr::RShift(a, n)    => eval_op(a, circ, memo) >> n,
    } & 0xFFFF; // stay in 16-bit

    memo.insert(reg.to_string(), out);
    out
}

pub fn p1(input: &str) -> i64 {
    let mut circ = HashMap::<String, Expr>::new();
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (dst, expr) = parse_line(line.trim_end_matches('\r'));
        circ.insert(dst, expr);
    }
    let mut memo = HashMap::<String, u16>::new();
    val("a", &circ, &mut memo) as i64
}

pub fn p2(input: &str) -> i64 {
    let first_part_solution = 46065u16;
    let mut circ = HashMap::<String, Expr>::new();
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (dst, expr) = parse_line(line.trim_end_matches('\r'));
        circ.insert(dst, expr);
    }
    // override b
    circ.insert("b".to_string(), Expr::Assign(Operand::Imm(first_part_solution)));

    let mut memo = HashMap::<String, u16>::new();
    val("a", &circ, &mut memo) as i64
}

