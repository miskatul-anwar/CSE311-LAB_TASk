use std::{
    io::{stdin, BufRead},
    mem::swap,
};

fn _rin() -> Vec<i32> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

/*  NOTE: implementation of bubble sort algorithm */
fn bubble_sort(data: &mut Vec<i32>, n: usize) {
    for k in 1..=n - 1 {
        let mut ptr: usize = 0;
        while ptr <= n - k {
            if data[ptr] < data[ptr] + 1 {
                swap(&mut data[ptr], &mut data[ptr + 1]);
            }
            ptr += 1;
        }
    }
}

fn main() {
    let mut data = _rin();
    let n: usize = data.len();

    bubble_sort(&mut data, n);
    println!("{data:?}");
}
