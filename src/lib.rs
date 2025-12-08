pub mod vp_tree;
pub mod y2015;
pub mod y2022;
pub mod y2025;

pub fn lookup(y: usize, d: usize) -> (fn(&str) -> i64, fn(&str) -> i64){
    let d = d - 1;
    match y {
        2015 => y2015::LOOKUP[d],
        2022 => y2022::LOOKUP[d],
        2025 => y2025::LOOKUP[d],
        _ => unimplemented!(),
    }
}
