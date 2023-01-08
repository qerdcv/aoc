mod error;
mod days;

use crate::days::day_1::*;

fn main() -> Result<(), error::Error> {
    println!("PART 1: {}", solve_p1()?);
    println!("PART 2: {}", solve_p2()?);

    Ok(())
}
