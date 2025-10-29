const GRID_WIDTH: usize = 1000;
const GRID_HEGIHT: usize = 1000;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEGIHT;

#[derive(Debug)]
enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}

pub fn p1(input: &str) -> i64 {
    let mut lights: [u8; GRID_SIZE] = [0u8; GRID_SIZE];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut from_part_idx = 1;
        let operation = match parts[0] {
            "toggle" => Operation::Toggle,
            "turn" => {
                from_part_idx = 2;
                if parts[1] == "on" {
                    Operation::TurnOn
                } else {
                    Operation::TurnOff
                }
            }
            op => panic!("unknown operation: {}", op),
        };

        let from_coords: Vec<usize> = parts[from_part_idx]
            .split(|c| c == ',')
            .map(|coord| coord.parse().expect("failed to parse coord"))
            .collect();

        let to_coords: Vec<usize> = parts[parts.len() - 1]
            .split(|c| c == ',')
            .map(|coord| coord.parse().expect("failed to parse coord"))
            .collect();

        for y in from_coords[1]..=to_coords[1] {
            let base = y * GRID_WIDTH;
            for x in from_coords[0]..=to_coords[0] {
                let idx = base + x;
                lights[idx] = match operation {
                    Operation::TurnOn => 1,
                    Operation::TurnOff => 0,
                    Operation::Toggle => lights[idx] ^ 1,
                };
            }
        }
    }

    lights.iter().map(|&l| l as i64).sum()
}

pub fn p2(input: &str) -> i64 {
    let mut lights: [u8; GRID_SIZE] = [0u8; GRID_SIZE];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut from_part_idx = 1;
        let operation = match parts[0] {
            "toggle" => Operation::Toggle,
            "turn" => {
                from_part_idx = 2;
                if parts[1] == "on" {
                    Operation::TurnOn
                } else {
                    Operation::TurnOff
                }
            }
            op => panic!("unknown operation: {}", op),
        };

        let from_coords: Vec<usize> = parts[from_part_idx]
            .split(|c| c == ',')
            .map(|coord| coord.parse().expect("failed to parse coord"))
            .collect();

        let to_coords: Vec<usize> = parts[parts.len() - 1]
            .split(|c| c == ',')
            .map(|coord| coord.parse().expect("failed to parse coord"))
            .collect();

        for y in from_coords[1]..=to_coords[1] {
            let base = y * GRID_WIDTH;
            for x in from_coords[0]..=to_coords[0] {
                let idx = base + x;
                lights[idx] = match operation {
                    Operation::TurnOn => lights[idx] + 1,
                    Operation::TurnOff => {
                        if lights[idx] != 0 {
                            lights[idx] - 1
                        } else {
                            0
                        }
                    }
                    Operation::Toggle => lights[idx] + 2,
                };
            }
        }
    }

    lights.iter().map(|&l| l as i64).sum()
}
