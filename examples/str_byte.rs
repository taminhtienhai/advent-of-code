fn main() {
    let bytes = include_bytes!("../resources/day03.txt");

    let total = bytes.split(|b| *b == b'\n')
    .map(|l| l.split_at(l.len() / 2))
    .map(|(a, b)| b
        .iter()
        .filter(|b| a.contains(b))
        .map(|b| if *b >= b'a' {
            (b - b'a') as i16 + 1
        } else {
            (b - b'A') as i16 + 27
        })
        .next()
        .unwrap())
    .sum::<i16>();


    println!("{total}");
}