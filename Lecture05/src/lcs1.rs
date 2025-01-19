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

/*NOTE: implementation of binary search algorithm*/
fn binary(data: &Vec<i32>, item: i32) {
    let mut beg = 0;
    let mut end = data.len();
    let mut mid = (beg + end) / 2;

    while beg <= end && data[mid as usize] != item {
        if item < data[mid as usize] {
            end = mid - 1;
        } else {
            beg = mid + 1;
        }
        mid = (beg + end) / 2;
    }

    if data[mid as usize] == item {
        println!("Loc = {mid}");
    } else {
        println!("{item} is not in the list!");
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let data = scanner._rin();
    let item = scanner._rin_int();
    binary(&data, item);
}
