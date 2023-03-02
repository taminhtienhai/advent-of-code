use std::{fs::File, io::{BufReader, Error, Read}};

pub fn day03() -> Result<i16, Error> {
    let file = File::open("resources/day03.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    
    reader.read_to_end(&mut buffer)?;

    let total = buffer.split(|byte| *byte == b'\n')
        .map(|item| item.split_at(item.len() / 2))
        .map(|(a, b)| {
            // let mut result = Vec::new();
            b.into_iter()
                .filter(|it| a.contains(it))
                .map(|item| if *item > b'a' {
                    (*item - b'a') as i16 + 1
                } else {
                    (*item - b'A') as i16 + 27
                })
                .next()
                .unwrap()
        })
        .sum();


    Ok(total)
}