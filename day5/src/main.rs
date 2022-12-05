fn main() {
    let input = include_str!("../input.txt");
    let Some((staks_str, moves)) = input.split_once("\n\n") else {
        panic!();
    };

    let mut staks: [Vec<char>; 10] = Default::default();

    for line in staks_str.lines().rev().skip(1) {
        let chars = line.trim_end().chars().skip(1).step_by(4);
        for (idx, c) in chars.enumerate() {
            if c != ' ' {
                staks[idx].push(c);
            }
        }
    }

    let moves = moves.lines().map(|l| l.into()).collect::<Vec<Move>>();

    solve2(moves, &mut staks);

    let res = staks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .collect::<String>();
    println!("{res}");
}

fn solve2(moves: Vec<Move>, staks: &mut [Vec<char>; 10]) {
    let mut temp = vec![];
    for Move { count, from, to } in moves {
        for _ in 0..count {
            if let Some(val) = staks[from].pop() {
                temp.push(val);
            }
        }
        temp.reverse();
        staks[to].append(&mut temp);
    }
}

fn solve1(moves: Vec<Move>, staks: &mut [Vec<char>; 10]) {
    for Move { count, from, to } in moves {
        for _ in 0..count {
            if let Some(val) = staks[from].pop() {
                staks[to].push(val);
            }
        }
    }
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let numbers = s
            .trim()
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|subs| subs.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Move {
            count: numbers[0],
            from: numbers[1] - 1,
            to: numbers[2] - 1,
        }
    }
}
