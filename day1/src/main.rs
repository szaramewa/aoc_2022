fn main() {
    let input = include_str!("../input.txt");
    let mut result = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    result.sort();

    let result = result.into_iter().rev().take(3).sum::<usize>();

    println!("{result:?}");
}
