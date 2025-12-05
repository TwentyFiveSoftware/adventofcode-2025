use std::cmp::max;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let database = Database::from_str(input).unwrap();

    database
        .available_ingredients
        .iter()
        .filter(|ingredient| database.is_ingredient_fresh(**ingredient))
        .count()
}

fn part2(input: &str) -> u64 {
    let database = Database::from_str(input).unwrap();

    let mut ranges = database.fresh_ingredient_ranges;
    ranges.sort_by(|(start1, end1), (start2, end2)| start1.cmp(start2).then(end1.cmp(end2)));

    let mut i = 0;
    while i < ranges.len() - 1 {
        let (start1, end1) = ranges[i];
        let (start2, end2) = ranges[i + 1];

        let does_overlap = start2 <= end1;
        if does_overlap {
            ranges[i] = (start1, max(end1, end2));
            ranges.remove(i + 1);
            continue;
        }

        i += 1;
    }

    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

struct Database {
    fresh_ingredient_ranges: Vec<(u64, u64)>,
    available_ingredients: Vec<u64>,
}

impl Database {
    fn is_ingredient_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ingredient_ranges
            .iter()
            .any(|(start, end)| ingredient >= *start && ingredient <= *end)
    }
}

impl FromStr for Database {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fresh, available) = s.split_once("\n\n").ok_or_else(|| ())?;

        Ok(Database {
            fresh_ingredient_ranges: fresh
                .lines()
                .map(|line| {
                    let (start, end) = line.split_once('-').unwrap();
                    (start.parse().unwrap(), end.parse().unwrap())
                })
                .collect(),
            available_ingredients: available
                .lines()
                .map(|line| line.parse().unwrap())
                .collect(),
        })
    }
}
