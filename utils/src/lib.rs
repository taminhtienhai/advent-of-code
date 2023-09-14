use std::{fs::File, io::{BufReader, Read}, error::Error, path::Path};


pub fn read_file_bytes(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::with_capacity(reader.capacity());

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        match read_file_bytes("../resources/day01.txt") {
            Ok(bytes) => println!("{bytes:?}"),
            Err(err) => println!("{err}"),
        }
    }
}
