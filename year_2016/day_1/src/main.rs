extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug)]
struct Error;

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error
    }
}

fn solve_p1() -> Result<isize, Error> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut current_direction: i16 = 0;
    let mut x_dist: isize = 0;
    let mut y_dist: isize = 0;

    buf.split(", ").for_each(
        |x| {
            let direction = &x[0..1];
            let val: isize = match (&x[1..x.len()])
                .parse() {
                Ok(num) => num,
                Err(_) => 0, // TODO: learn how to work with errors
            };

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
        }
    );
    Ok(y_dist.abs() + x_dist.abs())
}

fn solve_p2() -> Result<isize, Error> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut current_direction: i16 = 0;
    let mut x_pos: isize = 0;
    let mut y_pos: isize = 0;

    buf.split(", ").for_each(
        |x| {
            let direction = &x[0..1];
            let val: isize = match (&x[1..x.len()])
                .parse() {
                Ok(num) => num,
                Err(_) => 0, // TODO: learn how to work with errors
            };

            match direction {
                "R" => current_direction += 90,
                "L" => current_direction -= 90,
                _ => panic!("Unknown direction {}", direction),
            }

            current_direction = current_direction.rem_euclid(360);

            for _ in 0..val {
                match current_direction {
                    0 => y_pos += 1, // up
                    90 => x_pos += 1, // right
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
    );

    Ok(x_pos.abs() + y_pos.abs())
}

fn main() -> Result<(), Error> {
    println!("PART 1: {}", solve_p1()?);
    println!("PART 2: {}", solve_p2()?);

    Ok(())
}
