fn main() {
    let input = include_str!("../input.txt");
    let range_pairs: Vec<(Range_, Range_)> = input
        .lines()
        .map(|l| {
            let Some((r1, r2)) = l.split_once(',') else {
                panic!("chuj")
            };
            (r1.into(), r2.into())
        })
        .collect();

    let res = range_pairs
        .iter()
        .filter(|(r1, r2)| one_contains_other(r1, r2))
        .count();

    let res = range_pairs
        .iter()
        .filter(|(r1, r2)| r2.overlaps_with(&r1) || r1.overlaps_with(&r2))
        .count();

    println!("{res}");
}

pub fn one_contains_other(r1: &Range_, r2: &Range_) -> bool {
    r1.contains(&r2) || r2.contains(&r1)
}

pub struct Range_ {
    start: usize,
    end: usize,
}

impl Range_ {
    pub fn contains(&self, other: &Range_) -> bool {
        // other.start <= self.start && other.end <= self.start
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps_with(&self, other: &Range_) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

impl From<&str> for Range_ {
    fn from(s: &str) -> Self {
        let Some((start, end)) = s.split_once('-') else {
            panic!("dupa")
        };
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();

        assert!(start <= end);
        Self { start, end }
    }
}
