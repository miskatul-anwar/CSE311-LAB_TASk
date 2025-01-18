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

/* NOTE: implementation of bubble sort algorithm */
fn bubble_sort(data: &mut Vec<i32>, n: usize) {
    for k in 1..n {
        let mut ptr: usize = 0;
        while ptr < n - k {
            if data[ptr] > data[ptr + 1] {
                let temp = data[ptr];
                data[ptr] = data[ptr + 1];
                data[ptr + 1] = temp;
            }
            ptr += 1;
        }
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let mut data = scanner._rin();
    let n = data.len();
    bubble_sort(&mut data, n);
    println!("{:?}", data);
}
