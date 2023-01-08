extern crate core;

use crate::error::Error;
use std::collections::HashSet;
use std::fs::File;
use std::result::Result;
use std::io::Read;

pub fn solve_p1() -> Result<i16, Error> {
    let mut file = File::open("inputs/day_1.txt")?;
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

pub fn solve_p2() -> Result<i16, Error> {
    let mut file = File::open("inputs/day_1.txt")?;
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

    Ok(x_pos.abs() + y_pos.abs())
}
