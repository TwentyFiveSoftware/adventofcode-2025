use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|bank| bank.calculate_largest_power(2))
        .sum()
}

fn part2(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|bank| bank.calculate_largest_power(12))
        .sum()
}

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|line: &str| Bank::from_str(line).unwrap())
        .collect()
}

#[derive(Debug)]
struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn calculate_largest_power(&self, number_of_batteries: usize) -> u64 {
        let mut largest_power = 0;
        let mut min_battery_index = 0;

        for i in 0..number_of_batteries {
            let remaining_batteries = &self.batteries
                [min_battery_index..=self.batteries.len() - (number_of_batteries - i)];

            let (index, power) = Self::find_biggest_battery(remaining_batteries);

            min_battery_index += index + 1;
            largest_power = largest_power * 10 + power as u64;
        }

        largest_power
    }

    fn find_biggest_battery(batteries: &[u8]) -> (usize, u8) {
        let mut index = usize::MAX;
        let mut max_number = 0;

        for i in 0..batteries.len() {
            if batteries[i] > max_number {
                max_number = batteries[i];
                index = i;
            }
        }

        (index, max_number)
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank {
            batteries: s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(),
        })
    }
}
