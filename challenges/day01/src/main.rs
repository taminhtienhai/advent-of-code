use std::io::Error;

use day01::{part_01, part_02};
use utils::read_file_bytes;

fn main() -> Result<(), Error> {
    let max_01 = match read_file_bytes("resources/day01.txt") {
        Ok(bytes) => part_01::solution_01(bytes),
        _ => 0 as u32,
    };
    let max_02 = match read_file_bytes("resources/day01.txt") {
        Ok(bytes) => part_01::solution_02(bytes),
        _ => 0 as u32,
    };

    println!("Par01 solution_01: {max_01}");
    println!("Par02 solution_02: {max_02}");

    let part01_sol01 = match read_file_bytes("resources/day01.txt") {
        Ok(bytes) => part_02::solution_01(bytes),
        _ => 0 as u32,
    };

    println!("Par02 solution_01: {part01_sol01}");


    Ok(())
}
