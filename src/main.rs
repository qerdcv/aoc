use std::{error::Error, time::Instant};

use aoc::lookup;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("Expected: aoc {{problem id(y<YEAR>d<D>)}}");
    }

    args.next();
    let day_version = args.next().expect("problem version specification expected");
    let (year, day) = (
        day_version[1..5].parse::<usize>()?,
        day_version[6..].parse::<usize>()?
    );

    let path = format!("input/y{}/d{:02}.txt", year, day);
    dbg!(&path);
    let input = std::fs::read_to_string(path)?;

    let (p1, p2) = lookup(year, day);
    let start = Instant::now();
    println!("Day {} part one: {} . Elapsed: {:?}", day_version, p1(&input), start.elapsed());
    let start = Instant::now();
    println!("Day {} part two: {} . Elapsed: {:?}", day_version, p2(&input), start.elapsed());

    Ok(())
}
