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

/* NOTE: implementation of the insert function */
fn insert(la: &mut Vec<i32>, n: &mut usize, k: usize, item: i32) {
    let mut j = *n;

    la.push(0);

    while j > k {
        la[j] = la[j - 1];
        j -= 1;
    }

    la[k] = item;
    *n += 1;
}

fn main() {
    let mut scanner = Scanner::new();

    let mut la = scanner._rin();
    let item = scanner._rin_int();
    let k = scanner._rin_int() as usize;

    let mut n = la.len();
    insert(&mut la, &mut n, k, item);

    println!("{:?}", la);
}
