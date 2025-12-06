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

    let highlight_seq = "\x1b[30;47m";
    let res = "\x1b[0m";

    let (p1, p2) = lookup(year, day);
    let start = Instant::now();
    let (r1, e1) = (p1(&input), start.elapsed());
    
    let start = Instant::now();
    let (r2, e2) = (p2(&input), start.elapsed());

    let left1 = format!("Day {day_version} part one:{highlight_seq} {r1} {res}");
    let left2 = format!("Day {day_version} part two:{highlight_seq} {r2} {res}");

    let width = left1.len().max(left2.len());

    println!(
        "{:<width$} Elapsed: {:?}",
        left1,
        e1,
    );

    println!(
        "{:<width$} Elapsed: {:?}",
        left2,
        e2,
    );

    println!();
    Ok(())
}
