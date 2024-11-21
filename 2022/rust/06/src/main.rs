use std::{collections::HashSet, fs::read_to_string};

fn find_marker(contents: &str, n: usize) -> Option<usize> {
    contents
        .as_bytes()
        .windows(n)
        .position(|win| win.iter().collect::<HashSet<_>>().len() == n)
        .map(|i| i + n)
}

fn main() {
    let contents = read_to_string("input.txt").expect("Failed to read file input");
    let part_1 = find_marker(&contents, 4).unwrap_or(0);
    let part_2 = find_marker(&contents, 14).unwrap_or(0);

    println!("{part_1}");
    println!("{part_2}");
}
