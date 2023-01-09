use std::fs::File;
use std::io::Read;
use crate::error::Error;

#[allow(dead_code)]
pub fn solve_p1() -> Result<u16, Error>{
    let mut triangle_cnt: u16 = 0;
    let mut file = File::open("inputs/day_3.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    for line in buf.split('\n') {
        let sides: Vec<&str> = line.trim().split(' ').collect::<Vec<&str>>();
        let a: u16 = sides.get(0).expect("Should have number").parse()?;
        let b: u16 = sides.get(1).expect("Should have number").parse()?;
        let c: u16 = sides.get(2).expect("Should have number").parse()?;

        if a + b > c && a + c > b && c + b > a {
            triangle_cnt += 1;
        }
    }

    Ok(triangle_cnt)
}

#[allow(dead_code)]
pub fn solve_p2() -> Result<u16, Error>{
    let mut triangle_cnt: u16 = 0;
    let mut file = File::open("inputs/day_3.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let values = buf.split('\n').map(|line| {
        let sides: Vec<&str> = line.trim().split(' ').collect::<Vec<&str>>();
        let a: u16 = sides.get(0)
            .expect("Should have number")
            .parse()
            .expect("invalid number");
        let b: u16 = sides.get(1)
            .expect("Should have number")
            .parse()
            .expect("invalid number");
        let c: u16 = sides.get(2)
            .expect("Should have number")
            .parse()
            .expect("invalid number");
        [a, b, c]
    }).collect::<Vec<[u16; 3]>>();

    for i in 0..3 {
        for j in (0..values.len()).step_by(3) {
            let res = &values[j..j+3];
            let a = res[0].get(i).expect("must be by index");
            let b = res[1].get(i).expect("must be by index");
            let c = res[2].get(i).expect("must be by index");

            if a + b > *c && a + c > *b && c + b > *a {
                triangle_cnt += 1;
            }
        }
    }

    Ok(triangle_cnt)
}