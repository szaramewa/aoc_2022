#[derive(PartialEq, Eq)]
enum Shape {
    Rok = 1,
    Papjer,
    Scizor,
}

impl Shape {
    fn match_score(&self, other: Self) -> usize {
        match (self, other) {
            (s, o) if *s == o => 3,
            (Shape::Papjer, Shape::Rok) => 6,
            (Shape::Rok, Shape::Scizor) => 6,
            (Shape::Scizor, Shape::Papjer) => 6,
            (_, _) => 0,
        }
    }

    fn total_score(self, other: Self) -> usize {
        self.match_score(other) + self as usize
    }

    fn looses_to(&self) -> Self {
        match *self {
            Shape::Rok => Shape::Papjer,
            Shape::Papjer => Shape::Scizor,
            Shape::Scizor => Shape::Rok,
        }
    }

    fn wins_over(&self) -> Self {
        match *self {
            Shape::Rok => Shape::Scizor,
            Shape::Papjer => Shape::Rok,
            Shape::Scizor => Shape::Papjer,
        }
    }

    fn part_two(self, other: Self) -> usize {
        match self {
            Shape::Papjer => 3 + other as usize,
            Shape::Rok => 0 + other.wins_over() as usize,
            Shape::Scizor => 6 + other.looses_to() as usize,
        }
    }
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Shape::Rok,
            "B" | "Y" => Shape::Papjer,
            "C" | "Z" => Shape::Scizor,
            _ => unreachable!("zjebaues"),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let res = input
        .lines()
        .map(|l| {
            let Some((opp,me)) = l.split_once(' ') else {
                panic!("znowu zjebaues")
            };
            let opp = Shape::from(opp);
            let me = Shape::from(me);
            me.total_score(opp)
        })
        .sum::<usize>();

    let res = input
        .lines()
        .map(|l| {
            let Some((opp,me)) = l.split_once(' ') else {
                panic!("znowu zjebaues")
            };
            let opp = Shape::from(opp);
            let me = Shape::from(me);
            me.part_two(opp)
        })
        .sum::<usize>();

    println!("{res}");
}
