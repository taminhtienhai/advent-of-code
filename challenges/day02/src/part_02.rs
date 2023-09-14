pub fn solution01<'a>(bytes: impl IntoIterator<Item = &'a [u8]>) -> u32 {
    bytes.into_iter()
        .map(|chunk| {
            let op = chunk[0] - 64;
            let me = chunk[2] - 88;

            (
                if me == 0 { if op == 1 { 3 } else { op - 1 } }
                else if me == 1 { me * 3 + op }
                else { me * 3 + if op == 3 { 1 } else { op + 1 } }
            ) as u32
        }).sum::<u32>()
}