use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../input.txt");
    let res = solve(input, 14);

    println!("{}", res);
}

fn solve(input: &str, distinct_items: usize) -> usize {
    let bytes = input.as_bytes();

    let mut windows = bytes
        .windows(distinct_items)
        .enumerate()
        .filter(|(_, w)| {
            let set = BTreeSet::from_iter(w.iter());
            set.len() == w.len()
        })
        .map(|(idx, _)| idx);

    let window_idx = windows.next().unwrap();

    let res = window_idx + distinct_items;

    res
}
