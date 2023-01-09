extern crate core;

mod error;
mod days;

use crate::error::Error;
use crate::days::day_4::*;

fn main() -> Result<(), Error> {
    println!("PART 1: {}", solve_p1()?);
    println!("PART 2: {}", solve_p2()?);
    Ok(())
}
