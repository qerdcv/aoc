extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::{fmt, io};
use std::fmt::Formatter;
use std::result::Result;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug)]
struct Error {
    message: String
}

impl Error {
    fn new(message: String) -> Self {
        return Error{message}
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::new(value.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::new(value.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


fn solve_p1() -> Result<i16, Error> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut current_direction: i16 = 0;
    let mut x_dist: i16 = 0;
    let mut y_dist: i16 = 0;

    for x in buf.split(", ") {
        let direction = &x[0..1];
        let val: i16 = (&x[1..x.len()]).parse()?;

        match direction {
            "R" => current_direction += 90,
            "L" => current_direction -= 90,
            _ => panic!("Unknown direction {}", direction),
        }

        current_direction = current_direction.rem_euclid(360);
        match current_direction {
            0 => y_dist += val, // up
            90 => x_dist += val, // right
            180 => y_dist -= val, // down
            270 => x_dist -= val,// left
            _ => panic!("Unknown current direction {}", current_direction),
        }
    };

    Ok(y_dist.abs() + x_dist.abs())
}

fn solve_p2() -> Result<i16, Error> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut visited_positions: HashSet<(i16, i16)> = HashSet::new();
    let mut current_direction: i16 = 0;
    let mut x_pos: i16 = 0;
    let mut y_pos: i16 = 0;

    for x in buf.split(", ") {
        let direction = &x[0..1];
        let val: i16 = (&x[1..x.len()]).parse()?;

        match direction {
            "R" => current_direction += 90,
            "L" => current_direction -= 90,
            _ => panic!("Unknown direction {}", direction),
        }

        current_direction = current_direction.rem_euclid(360);

        for _ in 0..val {
            match current_direction {
                0   => y_pos += 1, // up
                90  => x_pos += 1, // right
                180 => y_pos -= 1, // down
                270 => x_pos -= 1,// left
                _ => panic!("Unknown current direction {}", current_direction),
            }

            let visited_position = (x_pos, y_pos);
            if visited_positions.contains(&visited_position) {
               break;
            }

            visited_positions.insert(visited_position);
        }

    }

    Ok(x_pos.abs() + y_pos.abs())
}

fn main() -> Result<(), Error> {
    println!("PART 1: {}", solve_p1()?);
    println!("PART 2: {}", solve_p2()?);

    Ok(())
}
