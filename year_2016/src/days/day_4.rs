use std::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use crate::error;

fn is_valid_room(name: &str, checksum: &str) -> bool {
    let mut c: HashMap<char, u16> = HashMap::new();
    name.replace('-', "").chars().for_each(|ch| {
        match c.get(&ch) {
            Some(val) => c.insert(ch, val + 1),
            _ => c.insert(ch, 1),
        };
    });

    let mut keys = c.keys().into_iter().collect::<Vec<&char>>();

    keys.sort();
    keys.sort_by(|a, b| c
        .get(b)
        .unwrap()
        .cmp(c.get(a).unwrap()));


    &keys.into_iter().collect::<String>()[0..5] == checksum
}

#[allow(dead_code)]
pub fn solve_p1() -> Result<u32, error::Error> {
    let re: Regex = Regex::new(r"([a-z-]+?)-(\d+)\[([a-z]{5})]").unwrap();
    let mut result: u32 = 0;
    let mut file = File::open("inputs/day_4.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    for line in buf.split('\n') {
        let capture = match re.captures(line) {
            Some(c) => c,
            None => panic!("No captures found"),
        };

        match (capture.get(1), capture.get(2), capture.get(3)) {
            (Some(name), Some(sector_id), Some(checksum)) => {

                if is_valid_room(name.as_str(), checksum.as_str()) {
                    let sector_id: u32 = sector_id.as_str().parse()?;
                    result += sector_id;
                }
            }
            _ => {}
        };
    }

    Ok(result)
}

#[allow(dead_code)]
pub fn solve_p2() -> Result<u32, error::Error> {
    const NORTH_POLE_OBJECT_STORAGE_NAME: &str = "northpole-object-storage";
    let re: Regex = Regex::new(r"([a-z-]+?)-(\d+)\[([a-z]{5})]")?;
    let mut file = File::open("inputs/day_4.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    for line in buf.split('\n') {
        let capture = match re.captures(line) {
            Some(c) => c,
            None => panic!("No captures found")
        };

        match (capture.get(1), capture.get(2), capture.get(3)) {
            (Some(name), Some(sector_id), Some(checksum)) => {
                if is_valid_room(name.as_str(), checksum.as_str()) {
                    let sector_id: u32 = sector_id.as_str().parse()?;
                    let delta: u8 = (sector_id % 26) as u8;
                    if name.as_str().chars().map(|ch| {
                        if !ch.is_ascii_alphabetic() {
                            ch
                        } else {
                            (((ch as u8) - 97 + delta) % 26 + 97 ) as char
                        }
                    }).collect::<String>() == NORTH_POLE_OBJECT_STORAGE_NAME {
                        return Ok(sector_id);
                    }
                }
            }
            _ => {}
        };
    }

    Ok(0)
}