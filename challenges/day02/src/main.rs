use std::{fs::File, io::{BufReader, Error, BufRead, Read}};

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

fn main() -> Result<(), Error> {
    println!("day 02");

    let file = File::open("resources/day02.txt")?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    let mut total_point = 0;

    reader.read_line(&mut content)?;

    while content.len() > 0 {
        if content.eq(&"\n") {
            content.clear();
            reader.read_line(&mut content)?;
            continue;
        }

        let pair = content.chars()
            .filter(|it| !it.eq(&' ') & !it.eq(&'\n'))
            .collect::<Vec<_>>();

        let enemy = pair.first().unwrap();
        let mine = pair.last().unwrap();

        let round = Round::from((enemy.to_owned(), mine.to_owned()));


        total_point += round.point();

        content.clear();
        reader.read_line(&mut content)?;
    }

    println!("Total point: {total_point:?}");

    assert_eq!(total_point, 15);

    Ok(())
}
