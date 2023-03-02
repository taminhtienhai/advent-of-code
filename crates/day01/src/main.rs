use std::{fs::{File}, path::Path, io::{Error, BufReader, BufRead}};

fn main() -> Result<(), Error> {
    let file = File::open(Path::new("resources/day01.txt"))?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut chunk = Vec::<u16>::new();
    let mut report = Vec::<u16>::new();

    reader.read_line(&mut buffer)?;

    while buffer.len() > 0 {
        let calories = buffer.split("\n")
            .take(1)
            .next().unwrap()
            .parse::<u16>()
            .unwrap_or(0);
        chunk.push(calories);

        if buffer.eq("\n"){
            if chunk.len() > 0 {
                let total = chunk.iter().sum();
                report.push(total);
            }
            chunk.clear();
        }
        buffer.clear();
        reader.read_line(&mut buffer)?;
    }

    println!("Maximum calories: {:?}", report.iter().max().unwrap());

    Ok(())
}