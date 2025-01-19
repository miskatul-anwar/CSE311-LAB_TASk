fn insert(mut text: Vec<char>, pos: usize, subs: Vec<char>) -> String {
    let mut j = 0;
    for i in 0..text.len() {
        if i >= pos - 1 && j < subs.len() {
            text.push(text[i]);
            text.insert(i, subs[j as usize]);
            j += 1;
        }
    }

    let mut ret = String::new();

    for i in 0..text.len() {
        ret.push(text[i]);
    }
    ret
}

fn main() {
    let text = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let pos = 3 as usize;
    let subs = vec!['X', 'Y', 'Z'];
    let text = insert(text, pos, subs);
    println!("New String: {text:?}");
}
