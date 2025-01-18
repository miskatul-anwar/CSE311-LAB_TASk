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
    let data: Vec<i32> = vec![8, 2, 5, 9, 1, 4, 7, 10, 3, 6];
    let item = 9;
    let n = data.len();
    linear(data, n, item);
}
