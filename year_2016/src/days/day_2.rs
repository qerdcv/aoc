use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::error::Error;

pub fn solve_p1() -> Result<(), Error> {
    const KEYMAP_SIZE: usize = 3;
    static KEYMAP: [u8; KEYMAP_SIZE * KEYMAP_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut file = File::open("inputs/day_2.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut x: usize = 1;
    let mut y: usize = 1;

    buf.split('\n').for_each(|line| {
        for ch in line.chars() {
            match ch {
                'U' => {
                    if y != 0 {
                        y -= 1
                    }
                },
                'L' => {
                    if x != 0 {
                        x -= 1
                    }
                },
                'D' => {
                    if y != 2 {
                        y += 1
                    }
                },
                'R' => {
                    if x != 2 {
                        x += 1
                    }
                },
                _ => {},
            }
        }

        print!("{}", KEYMAP[KEYMAP_SIZE *  y + x])
    });

    println!();
    Ok(())
}

pub fn solve_p2() -> Result<(), Error> {
    let keymap: HashMap<(usize, usize), char> = HashMap::from([
        // x, y, char
        ((2, 0), '1'),

        ((1, 1), '2'),
        ((2, 1), '3'),
        ((3, 1), '4'),

        ((0, 2), '5'),
        ((1, 2), '6'),
        ((2, 2), '7'),
        ((3, 2), '8'),
        ((4, 2), '9'),


        ((1, 3), 'A'),
        ((2, 3), 'B'),
        ((3, 3), 'C'),

        ((2, 4), 'D'),
    ]);

    let mut file = File::open("inputs/day_2.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut x: usize = 0;
    let mut y: usize = 2;

    buf.split('\n').for_each(|line| {
        for ch in line.chars() {
            let deny_vertical = (x == 0 || x == 4) && y == 2;
            let deny_horizontal = (y == 0 || y == 4) && x == 2;
            match ch {
                'U' => {
                    if ((y != 1 && x != 2) || (y != 0 && x == 2)) && !deny_vertical {
                        y -= 1
                    }
                },
                'L' => {
                    if ((x != 1 && y != 2) || (x != 0 && y == 2)) && !deny_horizontal {
                        x -= 1
                    }
                },
                'D' => {
                    if ((y != 3 && x != 2) || (y != 4 && x == 2)) && !deny_vertical {
                        y += 1
                    }
                },
                'R' => {
                    if ((x != 3 && y != 2) || (x != 4 && y == 2)) && !deny_horizontal {
                        x += 1
                    }
                },
                _ => {},
            }
        }

         match keymap.get(&(x, y)) {
             Some(val) => print!("{}", val),
             None => panic!("unknown key {}-{}", x, y),
         };
    });

    println!();
    Ok(())
}
