use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}

fn part_two(input: &str) {
    let lines = input
        .lines()
        .map(|l| BTreeSet::from_iter(l.chars()))
        .collect::<Vec<_>>();
    let three_common = lines
        .chunks_exact(3)
        .flat_map(|sets| {
            sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            })
        })
        .collect::<Vec<char>>();

    let res = three_common.iter().map(score).sum::<u32>();
    println!("{res}");
}

fn part_one(input: &str) {
    let sets = input
        .lines()
        .map(|l| {
            let l = l.trim();
            let half = l.len() / 2;
            let (a, b) = l.split_at(half);
            (
                BTreeSet::from_iter(a.chars()),
                BTreeSet::from_iter(b.chars()),
            )
        })
        .collect::<Vec<(BTreeSet<char>, BTreeSet<char>)>>();
    let res = sets
        .iter()
        .map(|(a, b)| {
            let i = a.intersection(&b).cloned().collect::<Vec<char>>();
            i.iter().map(score).sum::<u32>()
        })
        .sum::<u32>();
    println!("{res}");
}

fn score(c: &char) -> u32 {
    let d = if c.is_ascii_lowercase() {
        *c as u8 - b'a' + 1
    } else {
        *c as u8 - b'A' + 27
    };
    d as u32
}
