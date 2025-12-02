use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let ranges = parse_input(input);

    let mut invalid_ids = vec![];

    for range in ranges {
        for number in range.start..=range.end {
            let number_string = number.to_string();
            if number_string.len() % 2 != 0 {
                continue;
            }

            let first_half = &number_string[..number_string.len() / 2];
            let second_half = &number_string[number_string.len() / 2..];

            if first_half == second_half {
                invalid_ids.push(number);
            }
        }
    }

    invalid_ids.iter().sum()
}

fn part2(input: &str) -> u64 {
    let ranges = parse_input(input);

    let mut invalid_ids = vec![];

    for range in ranges {
        for number in range.start..=range.end {
            let number_string = number.to_string();

            for substring_length in 1..=number_string.len() / 2 {
                let repeat_times = number_string.len() / substring_length;
                if repeat_times * substring_length != number_string.len() {
                    continue;
                }

                let substring = &number_string[..substring_length];
                if substring.repeat(repeat_times) == number_string {
                    invalid_ids.push(number);
                    break;
                }
            }
        }
    }

    invalid_ids.iter().sum()
}

fn parse_input(input: &str) -> Vec<IdRange> {
    input
        .split(",")
        .map(|range: &str| IdRange::from_str(range).unwrap())
        .collect()
}

struct IdRange {
    start: u64,
    end: u64,
}

impl FromStr for IdRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.split("-").collect::<Vec<&str>>();
        if x.len() != 2 {
            return Err(());
        }

        let start = x[0].parse::<u64>().map_err(|_| ())?;
        let end = x[1].parse::<u64>().map_err(|_| ())?;

        Ok(Self { start, end })
    }
}
