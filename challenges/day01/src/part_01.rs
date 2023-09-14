#[derive(Debug)]
pub struct CalorieCounter {
    newline: u32,
    max: u32,
    acc: u32,
    cur: u32,
}

impl CalorieCounter {
    pub fn new() -> Self {
        Self {
            newline: 0,
            max: 0,
            acc: 0,
            cur: 0,
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
        if total > self.max {
            self.max = total;
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

    return calorie_counter.eof().max;
}

pub fn solution_02(bytes: impl IntoIterator<Item = u8>) -> u32 {
    let CalorieCounter { newline: _newline, mut max, acc, cur } = bytes.into_iter().fold(
        CalorieCounter::new(),
        |CalorieCounter { newline, mut max, acc, cur }, it| if it == 10 {
            CalorieCounter { newline: newline + 1, max, acc, cur }
        } else if it != 10 && newline == 1 {
            CalorieCounter { newline: 0, max, acc: acc + cur, cur: (it - 48) as u32 }
        } else if it != 10 && newline < 1 {
            CalorieCounter { newline, max, acc, cur: cur * 10 + ((it - 48) as u32) }
        } else {
            let total = acc + cur;
            if total > max {
                max = total;
            }
            CalorieCounter { newline: 0, max, acc: 0, cur: (it - 48) as u32 }
        },
    );

    let total = acc + cur;
    if total > max {
        max = total;
    }

    return max;

}