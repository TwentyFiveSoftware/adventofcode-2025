#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let steps = parse_input(input);

    let mut dial = 50;
    let mut times_pointing_at_zero = 0;

    for (rotation, turns) in steps {
        let diff = match rotation {
            Direction::Left => -turns,
            Direction::Right => turns,
        };

        dial = (dial + diff) % 100;

        if dial == 0 {
            times_pointing_at_zero += 1;
        }
    }

    times_pointing_at_zero
}

fn part2(input: &str) -> i32 {
    let steps = parse_input(input);

    let mut dial = 50;
    let mut times_pointing_at_zero = 0;

    for (rotation, turns) in steps {
        for _ in 0..turns {
            match rotation {
                Direction::Left => {
                    dial -= 1;
                }
                Direction::Right => {
                    dial += 1;
                }
            }

            dial = dial % 100;

            if dial == 0 {
                times_pointing_at_zero += 1;
            }
        }
    }

    times_pointing_at_zero
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input.lines().map(|line: &str| {
        let rotation = if line.chars().next().unwrap_or(' ') == 'L' { Direction::Left } else { Direction::Right };
        let turns = line[1..].parse::<i32>().unwrap_or(0);

        (rotation, turns)
    }).collect()
}

enum Direction {
    Left,
    Right,
}
