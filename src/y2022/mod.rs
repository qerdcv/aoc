pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;

pub static LOOKUP: [(fn(&str) -> i64, fn(&str) -> i64); 12] = [
    (d01::p1, d01::p2),
    (d02::p1, d02::p2),
    (d03::p1, d03::p2),
    (d04::p1, d04::p2),
    (d05::p1, d05::p2),
    (d06::p1, d06::p2),
    (d07::p1, d07::p2),
    (d08::p1, d08::p2),
    (d09::p1, d09::p2),
    (d10::p1, d10::p2),
    (d11::p1, d11::p2),
    (d12::p1, d12::p2),
];
