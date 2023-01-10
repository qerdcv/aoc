use std::collections::HashMap;
use std::fs;
use crate::error::Error;

fn collect_lines() -> Result<Vec<Vec<char>>, Error> {
    Ok(fs::read_to_string("inputs/day_6.txt")?
        .split('\n')
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>())
}
fn collect_map(lines: &Vec<Vec<char>>, col: usize) -> HashMap<char, u32> {
    let mut c: HashMap<char, u32> = HashMap::new();
    for row in lines {
        if let Some(ch) = row.get(col) {
            let val = match c.get(ch) {
                Some(v) => v + 1,
                None => 1,
            };
            c.insert(*ch, val);
        }
    }

    c
}

fn get_most_repeated_char(m: HashMap<char, u32>) -> char {
    let mut keys = m.keys().into_iter().collect::<Vec<&char>>();
    keys.sort_by(|a, b| m.get(b).unwrap().cmp(m.get(a).unwrap()));

    **keys.get(0).unwrap()
}

fn get_least_repeated_char(m: HashMap<char, u32>) -> char {
    let mut keys = m.keys().into_iter().collect::<Vec<&char>>();
    keys.sort_by(|a, b| m.get(a).unwrap().cmp(m.get(b).unwrap()));

    **keys.get(0).unwrap()
}

#[allow(dead_code)]
pub fn solve_p1() -> Result<String, Error> {
    let lines = collect_lines()?;
    let mut result: [char; 8] = ['\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'];

    for col in 0..8 as usize {

        result[col] = get_most_repeated_char(collect_map(&lines, col));
    }

    Ok(result.iter().collect::<String>())
}

#[allow(dead_code)]
pub fn solve_p2() -> Result<String, Error> {
    let lines = collect_lines()?;
    let mut result: [char; 8] = ['\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'];

    for col in 0..8 as usize {
        result[col] = get_least_repeated_char(collect_map(&lines, col));
    }

    Ok(result.iter().collect::<String>())
}