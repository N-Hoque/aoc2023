#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use std::io::BufRead;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;

pub fn read_file(day: u8) -> Vec<String> {
    let data = std::fs::File::open(format!("res/day_{day}.txt")).unwrap();
    let reader = std::io::BufReader::new(data);
    reader.lines().map_while(Result::ok).collect::<Vec<_>>()
}
