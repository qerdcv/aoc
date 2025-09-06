use aoc::y2015d06::{p1, p2};

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("Expected: aoc {{problem id(y<YEAR>d<D>)}}");
    }

    args.next();

    let day_version = args.next().expect("problem version specification expected");
    let input =
        std::fs::read(format!("input/{}.txt", day_version)).expect("Failed to read day input");

    eprintln!("Day {} part one: {}", day_version, p1(&input));
    eprintln!("Day {} part two: {}", day_version, p2(&input));
}
