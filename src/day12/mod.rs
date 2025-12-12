#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let shape_sizes = parts[..parts.len() - 1]
        .iter()
        .map(|shape| shape[2..].chars().filter(|c| *c == '#').count())
        .collect::<Vec<_>>();

    let regions = parts[parts.len() - 1]
        .split("\n")
        .map(|line| {
            let (size, shape_counts) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once("x").unwrap();

            (
                width.parse::<usize>().unwrap() * height.parse::<usize>().unwrap(),
                shape_counts
                    .split(" ")
                    .map(|count| count.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    regions
        .into_iter()
        .filter(|(available_space, shape_counts)| {
            shape_counts
                .iter()
                .zip(shape_sizes.iter())
                .fold(0, |sum, (shape_count, shape_size)| {
                    sum + shape_count * shape_size
                })
                <= *available_space
        })
        .count()
}
