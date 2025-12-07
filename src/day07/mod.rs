use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    Grid::from_str(input).unwrap().number_of_splits()
}

fn part2(input: &str) -> usize {
    Grid::from_str(input).unwrap().number_of_timelines()
}

struct Grid {
    start: (i64, i64),
    height: i64,
    splitter: HashSet<(i64, i64)>,
}

impl Grid {
    fn number_of_splits(&self) -> usize {
        let mut splits = HashSet::new();

        let mut current_beam_heads = HashSet::new();
        current_beam_heads.insert(self.start);

        while !current_beam_heads.is_empty() {
            for (x, mut y) in current_beam_heads.clone() {
                current_beam_heads.remove(&(x, y));
                y += 1;

                if y > self.height {
                    current_beam_heads.remove(&(x, y));
                    continue;
                }

                if self.splitter.contains(&(x, y)) {
                    current_beam_heads.remove(&(x, y));

                    current_beam_heads.insert((x - 1, y));
                    current_beam_heads.insert((x + 1, y));

                    splits.insert((x, y));
                } else {
                    current_beam_heads.insert((x, y));
                }
            }
        }

        splits.len()
    }

    fn number_of_timelines(&self) -> usize {
        fn _number_of_timelines(
            splitter: &HashSet<(i64, i64)>,
            height: i64,
            cache: &mut HashMap<(i64, i64), usize>,
            (start_x, start_y): (i64, i64),
        ) -> usize {
            for y in start_y..height {
                if splitter.contains(&(start_x, y)) {
                    if cache.contains_key(&(start_x, y)) {
                        return *cache.get(&(start_x, y)).unwrap();
                    }

                    let timelines = _number_of_timelines(splitter, height, cache, (start_x - 1, y))
                        + _number_of_timelines(splitter, height, cache, (start_x + 1, y));

                    cache.insert((start_x, y), timelines);
                    return timelines;
                }
            }

            1
        }

        let mut cache = HashMap::new();
        _number_of_timelines(&self.splitter, self.height, &mut cache, self.start)
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines().enumerate().fold(
            Self {
                start: (0, 0),
                height: s.lines().count() as i64,
                splitter: HashSet::new(),
            },
            |mut grid, (y, line)| {
                line.chars().enumerate().for_each(|(x, char)| match char {
                    'S' => {
                        grid.start = (x as i64, y as i64);
                    }
                    '^' => {
                        grid.splitter.insert((x as i64, y as i64));
                    }
                    _ => {}
                });

                grid
            },
        ))
    }
}
