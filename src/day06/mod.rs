#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut math_problems = parse_math_problem_operations(input);

    input.lines().rev().skip(1).for_each(|line| {
        line.trim()
            .split_whitespace()
            .enumerate()
            .for_each(|(i, number)| {
                let value = number.parse().unwrap();
                math_problems[i].values.push(value);
            });
    });

    math_problems.iter().map(|problem| problem.solve()).sum()
}

fn part2(input: &str) -> i64 {
    let mut math_problems = parse_math_problem_operations(input);

    let lines = input
        .lines()
        .rev()
        .skip(1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    let max_column = lines.iter().map(|line| line.len()).max().unwrap();

    let values = (0..max_column)
        .map(|column| {
            let digit = lines
                .iter()
                .map(|line| line.chars().nth(column).unwrap_or(' '))
                .collect::<String>();

            if digit.trim().is_empty() {
                None
            } else {
                Some(digit.trim().parse::<i64>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    values
        .split(|item| item.is_none())
        .enumerate()
        .for_each(|(i, values)| {
            math_problems[i].values = values.iter().filter_map(|value| value.clone()).collect();
        });

    math_problems.iter().map(|problem| problem.solve()).sum()
}

fn parse_math_problem_operations(input: &str) -> Vec<MathProblem> {
    input
        .lines()
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|operation| match operation {
            "+" => Some(MathProblem {
                operation: Operation::Add,
                values: Vec::new(),
            }),
            "*" => Some(MathProblem {
                operation: Operation::Multiply,
                values: Vec::new(),
            }),
            &_ => None,
        })
        .collect::<Vec<_>>()
}

struct MathProblem {
    operation: Operation,
    values: Vec<i64>,
}

impl MathProblem {
    fn solve(&self) -> i64 {
        match self.operation {
            Operation::Add => self.values.iter().sum(),
            Operation::Multiply => self.values.iter().product(),
        }
    }
}

enum Operation {
    Add,
    Multiply,
}
