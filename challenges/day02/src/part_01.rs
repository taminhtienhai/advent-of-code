pub fn solution01<'a>(bytes: impl IntoIterator<Item = &'a [u8]>) -> u32 {
    bytes.into_iter()
        .map(|chunk| {
            let op = chunk[0] as i8 - 66;
            let me = chunk[2] as i8 - 89;
            (
                if op == me { 3 }
                else if op > 0 { if me < 0 { 6 } else { 0 } }
                else if op < 0 { if me > 0 { 0 } else { 6 } }
                else { if me > 0 { 6 } else { 0 } } + me + 2
            ) as u32
        }).sum::<u32>()
}