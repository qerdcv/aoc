use std::fs::File;
use std::io::Read;
use crate::error::Error;

#[allow(dead_code)]
pub fn solve_p1() -> Result<String, Error> {
    let mut file = File::open("inputs/day_5.txt")?;
    let mut buf = String::new();
    let mut result = String::new();
    file.read_to_string(&mut buf)?;

    let mut idx: u32 = 0;
    let mut found_cnt = 0;
    while found_cnt != 8 {
        let h = format!("{:x}", md5::compute(format!("{}{}", buf, idx)));
        if &h[0..5] == "00000" {
            println!("{}", idx);
            result += &h[5..6];
            found_cnt += 1;
        }

        idx += 1;
    }
    Ok(result)
}

#[allow(dead_code)]
pub fn solve_p2() -> Result<String, Error> { // TODO: add cinematic decrypting animation
    let mut file = File::open("inputs/day_5.txt")?;
    let mut buf = String::new();
    let mut result: [char; 8] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    file.read_to_string(&mut buf)?;

    let mut idx: u32 = 0;
    let mut found_cnt = 0;

    while found_cnt != 8 {
        let h = format!("{:x}", md5::compute(format!("{}{}", buf, idx)));
        let mut hash_chars = h.chars();
        let pos =  match hash_chars.nth(5) {
            Some(v) => v,
            None => panic!("End of iteration")
        };
        if &h[0..5] == "00000" && pos.is_numeric() {
            let idx: usize = match pos.to_digit(10) {
                Some(v) => v as usize,
                None => panic!("Not a number")
            };
            if idx < 8 && result[idx] == ' ' {
                let ch_to_insert = match hash_chars.next() {
                    Some(v) => v,
                    None => panic!("End of iteration"),
                };

                result[idx] = ch_to_insert;
                found_cnt += 1;
                println!("{}", result.iter().map(|ch| {
                    let ch = *ch;
                    if ch == ' ' {
                        '_'
                    } else {
                        ch
                    }
                }).collect::<String>());
            }
        }

        idx += 1;
    }

    Ok(result.into_iter().collect::<String>())
}