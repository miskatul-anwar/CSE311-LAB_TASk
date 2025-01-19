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

/* NOTE: Deleting elements at a specific index */
fn delete(la: &mut Vec<i32>, k: usize, n: usize) {
    for j in k..n - 1 {
        la[j] = la[j + 1];
    }
    la.pop();
}

fn main() {
    let mut scanner = Scanner::new();
    let mut la: Vec<i32> = scanner._rin();
    let k = scanner._rin_int() as usize;

    let item = la[k];
    let n = la.len();
    delete(&mut la, k, n);

    println!("Updated vector: {la:?}");
    println!("Deleted item: {}", item);
    println!("New Length: {}", la.len());
}
