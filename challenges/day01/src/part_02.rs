#[derive(Debug)]
pub struct CalorieCounter {
    newline: u32,
    acc: u32,
    cur: u32,
    top03: [u32; 3]
}

impl CalorieCounter {
    pub fn new() -> Self {
        Self {
            newline: 0,
            acc: 0,
            cur: 0,
            top03: [0,0,0],
        }
    }

    #[inline]
    pub fn breakline(mut self) -> Self {
        self.newline += 1;
        self
    }

    #[inline]
    pub fn fold_cur(mut self, it: u8) -> Self {
        self.newline = 0;
        self.acc = self.acc + self.cur;
        self.cur = (it - 48) as u32;
        self
    }

    #[inline]
    pub fn increment(mut self, it: u8) -> Self {
        self.cur = self.cur * 10 + ((it - 48) as u32);
        self
    }

    #[inline]
    pub fn accumulate(mut self, it: u8) -> Self {
        let total = self.acc + self.cur;
        if Some(&total) > self.top03.last() {
            self.laddering(total);
        }
        self.newline = 0;
        self.acc = 0;
        self.cur = (it - 48) as u32;
        self
    }

    pub fn eof(self) -> Self {
        if self.acc > 0 || self.cur > 0 {
            self.accumulate(48)
        } else {
            self
        }
    }

    #[inline]
    pub fn laddering(&mut self, total: u32) {
        let [first, second, third] = self.top03.clone();
        if third > total {
            return;
        } else if total > third && second > total {
            self.top03[2] = total;
        } else if total > second && first > total {
            self.top03[2] = first;
            self.top03[1] = total;
        } else {
            self.top03[2] = second;
            self.top03[1] = first;
            self.top03[0] = total;
        }
    }
}

pub fn solution_01(bytes: impl IntoIterator<Item = u8>) -> u32 {
    let calorie_counter = bytes.into_iter().fold(
        CalorieCounter::new(),
        |acc, it| if it == 10 {
            acc.breakline()
        } else if it != 10 && acc.newline == 1 {
            acc.fold_cur(it)
        } else if it != 10 && acc.newline < 1 {
            acc.increment(it)
        } else {
            acc.accumulate(it)
        },
    );


    return calorie_counter.eof().top03.iter().sum();
}