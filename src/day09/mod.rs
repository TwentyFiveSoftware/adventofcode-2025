use std::cmp::{max, min};

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    build_rectangles(&parse_points(input))
        .iter()
        .map(|rectangle| rectangle.area())
        .max()
        .unwrap_or(0)
}

fn part2(input: &str) -> i64 {
    let points = parse_points(input);
    let polygon = Polygon::from_points(&points);
    let rectangles = build_rectangles(&points);

    rectangles
        .iter()
        .filter(|rectangle| !polygon.intersects(rectangle))
        .map(|rectangle| rectangle.area())
        .max()
        .unwrap_or(0)
}

fn parse_points(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn build_rectangles(points: &Vec<(i64, i64)>) -> Vec<Rectangle> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, point1)| {
            points
                .iter()
                .skip(i + 1)
                .map(|point2| Rectangle::from_points(*point1, *point2))
        })
        .collect()
}

struct Rectangle {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Rectangle {
    fn from_points((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> Self {
        Self {
            min_x: min(x1, x2),
            max_x: max(x1, x2),
            min_y: min(y1, y2),
            max_y: max(y1, y2),
        }
    }

    fn area(&self) -> i64 {
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        !(self.min_x > other.max_x - 1
            || self.max_x < other.min_x + 1
            || self.max_y < other.min_y + 1
            || self.min_y > other.max_y - 1)
    }
}

struct Polygon {
    borders: Vec<Rectangle>,
}

impl Polygon {
    fn from_points(points: &Vec<(i64, i64)>) -> Self {
        Self {
            borders: (0..points.len())
                .map(|i| Rectangle::from_points(points[i], points[(i + 1) % points.len()]))
                .collect(),
        }
    }

    fn intersects(&self, rectangle: &Rectangle) -> bool {
        self.borders
            .iter()
            .any(|border| border.intersects(rectangle))
    }
}
