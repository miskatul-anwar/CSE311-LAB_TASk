use std::io::{self, BufRead};

struct Scanner {
    reader: Box<dyn BufRead>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            reader: Box::new(io::stdin().lock()),
        }
    }

    fn _rin(&mut self) -> Vec<i32> {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        input
            .split_whitespace()
            .map(|i| i.parse().expect("Failed to parse integer"))
            .collect()
    }

    fn _rin_int(&mut self) -> i32 {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        input.trim().parse().expect("Failed to parse integer")
    }
}

/*NOTE: implementation of linear search algorithm*/
fn linear(data: Vec<i32>, n: usize, item: i32) {
    let mut loc = 0;

    while data[loc as usize] != item {
        loc += 1;
    }

    if loc as usize == n + 1 {
        loc = -1;
    }

    if loc == -1 {
        println!("{item} is not in the list!");
    } else {
        println!("{item} found at pos: {loc}");
    }
}
fn main() {
    let mut scanner = Scanner::new();
    let data: Vec<i32> = scanner._rin();
    let item = scanner._rin_int();
    let n = data.len();
    linear(data, n, item);
}
