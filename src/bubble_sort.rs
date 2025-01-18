use std::{
    io::{stdin, BufRead},
    mem::replace,
};

fn _rin() -> Vec<i32> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
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
    // let mut data = _rin();
    let mut data = Vec::from([1, 2, 19, 1, 20, 2, 12, 4, 2, 43, 12, 45, 63, 32]);
    let n = data.len();
    bubble_sort(&mut data, n);
    println!("{:?}", data);
}
