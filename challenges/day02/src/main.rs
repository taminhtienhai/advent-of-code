mod part_01;
mod part_02;

use std::{fs::File, io::{BufReader, BufRead, Read}, error::Error};

use utils::read_file_bytes;

enum Strategy {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

struct Round(u8, u8);

impl From<(char,char)> for Round {

    fn from(values: (char, char)) -> Self {
        match values {
            ('A','X') => Round(1, 1),
            ('A','Y') => Round(1, 2),
            ('A','Z') => Round(1, 3),
            ('B','X') => Round(2, 1),
            ('B','Y') => Round(2, 2),
            ('B','Z') => Round(2, 3),
            ('C','X') => Round(3, 1),
            ('C','Y') => Round(3, 2),
            ('C','Z') => Round(3, 3),
            _ => panic!(),
        }
    }
}

impl Round {
    pub fn point(&self) -> u16 {
        let base_point = match self.0 as i16 - self.1 as i16 {
            x if x < 0 => 3,
            x if x == 0 => 0,
            _ => 6,
        } as u16;
        return self.1 as u16 + base_point;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = read_file_bytes("resources/day02.txt")?;

    let points = part_01::solution01(bytes.chunks(4));

    println!("Part 01: {points}");
    println!("Part 02: {}", part_02::solution01(bytes.chunks(4)));

    Ok(())
}
