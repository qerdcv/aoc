use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("unknown value: {}", value)
        }
    }
}

pub fn p1(input: &str) -> i64 {
    let input: Vec<(Dir, i64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_ascii_whitespace())
        .map(|mut parts| (
            Dir::from(parts.next().expect("No dir part")), 
            parts.next().expect("No count part").parse::<i64>().expect("Not a number"),
        ))
        .collect();

    let mut head_pos = Pos{x: 0, y: 0};
    let mut tail_pos = Pos{x: 0, y: 0};
    let mut visited = HashSet::new();
    visited.insert(tail_pos);
    for (dir, cnt) in input {
        for _ in 0..cnt {
            let prev_head_pos = head_pos;
            match dir {
                Dir::Up => head_pos.y += 1,
                Dir::Right => head_pos.x += 1,
                Dir::Down => head_pos.y -= 1,
                Dir::Left => head_pos.x -= 1,
            }

            if head_pos.x.abs_diff(tail_pos.x) > 1 || head_pos.y.abs_diff(tail_pos.y) > 1 {
                tail_pos = prev_head_pos;
                visited.insert(tail_pos);
            }
        }
    }


    visited.len() as i64
}

pub fn p2(input: &str) -> i64 {
    let input: Vec<(Dir, i64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_ascii_whitespace())
        .map(|mut parts| (
            Dir::from(parts.next().expect("No dir part")), 
            parts.next().expect("No count part").parse::<i64>().expect("Not a number"),
        ))
        .collect();

    let head = 0;
    let tail = 9;
    let mut knots = [
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
        Pos{x: 0, y: 0},
    ];
    let mut visited = HashSet::new();
    visited.insert(Pos{x: 0,y: 0});
    for (dir, cnt) in input {
        for _ in 0..cnt {
             match dir {
                Dir::Up => knots[head].y += 1,
                Dir::Right => knots[head].x += 1,
                Dir::Down => knots[head].y -= 1,
                Dir::Left => knots[head].x -= 1,
            }

            let mut prev_knot_pos  = knots[head];
            for k in &mut knots[1..] {
                let dx = prev_knot_pos.x - k.x;
                let dy = prev_knot_pos.y - k.y;

                if dx.abs() > 1 || dy.abs() > 1 {
                    k.x += dx.signum();
                    k.y += dy.signum();
                } else {
                    break;
                }

                prev_knot_pos = *k;
            }

            visited.insert(knots[tail]);
        }
    }


    visited.len() as i64
}
