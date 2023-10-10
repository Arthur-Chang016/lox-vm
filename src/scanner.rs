

pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

impl Scanner {
    pub fn new() -> Scanner {
        return Scanner {
            start: 0,
            current: 0,
            line: 1,
        }
    }
}


