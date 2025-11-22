use std::time::Instant;

use aoc::y2022::d04::{p1, p2};

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("Expected: aoc {{problem id(y<YEAR>d<D>)}}");
    }

    args.next();

    let day_version = args.next().expect("problem version specification expected");
    let path = format!("input/{}/{}.txt", &day_version[..5], &day_version[5..]);
    dbg!(&path);
    let input = std::fs::read_to_string(path).expect("Failed to read day input");

    let start = Instant::now();
    println!("Day {} part one: {}. Elapsed: {:?}", day_version, p1(&input), start.elapsed());
    let start = Instant::now();
    println!("Day {} part two: {}. Elapsed: {:?}", day_version, p2(&input), start.elapsed());
}
