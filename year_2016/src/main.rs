mod error;
mod days;

use crate::days::day_2::*;

fn main() -> Result<(), error::Error> {
    solve_p1()?;
    solve_p2()?;

    Ok(())
}
