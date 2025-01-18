use std::io::{stdin, BufRead};

fn _rin() -> i32 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn _vec_rin() -> Vec<i32> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
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
    let mut la = _vec_rin();
    let item = _rin();
    let k = _rin() as usize;

    let mut n = la.len();
    insert(&mut la, &mut n, k, item);

    println!("{:?}", la);
}
