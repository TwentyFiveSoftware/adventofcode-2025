use std::collections::HashSet;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    Grid::from_str(input).unwrap().accessible_roles().len()
}

fn part2(input: &str) -> u64 {
    let mut grid = Grid::from_str(input).unwrap();
    let mut removed_rolls = 0;

    loop {
        let accessible_roles = grid.accessible_roles();
        if accessible_roles.len() == 0 {
            break;
        }

        for position in accessible_roles {
            grid.rolls_of_paper.remove(&position);
            removed_rolls += 1;
        }
    }

    removed_rolls
}

#[derive(Debug)]
struct Grid {
    rolls_of_paper: HashSet<(i32, i32)>,
}

impl Grid {
    fn accessible_roles(&self) -> Vec<(i32, i32)> {
        self.rolls_of_paper
            .iter()
            .filter_map(|position| match self.is_role_accessible(*position) {
                true => Some(*position),
                false => None,
            })
            .collect()
    }

    fn is_role_accessible(&self, position: (i32, i32)) -> bool {
        self.number_of_adjacent_roles(position) < 4
    }

    fn number_of_adjacent_roles(&self, (x, y): (i32, i32)) -> usize {
        (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .map(|(x_offset, y_offset)| self.rolls_of_paper.contains(&(x + x_offset, y + y_offset)))
            .filter(|&p| p)
            .count()
            - 1
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            rolls_of_paper: s
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| match c {
                        '@' => Some((x as i32, y as i32)),
                        _ => None,
                    })
                })
                .collect(),
        })
    }
}
