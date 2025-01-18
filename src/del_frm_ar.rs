use std::io::{stdin, BufRead};

fn _rin() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn delete(la: &mut Vec<i32>, k: usize, n: usize) {
    for j in k..n - 1 {
        la[j] = la[j + 1];
    }
    la.pop();
}

fn main() {
    let mut la: Vec<i32> = vec![1, 2, 3, 4, 9];
    let k = _rin();

    let item = la[k];
    let n = la.len();
    delete(&mut la, k, n);

    println!("Updated vector: {la:?}");
    println!("Deleted item: {}", item);
    println!("New Length: {}", la.len());
}
