use std::collections::HashSet;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut circuits = Circuits::from_str(input).unwrap();

    for _ in 0..1000 {
        let (_, junction_a, junction_b) = circuits.distances.pop().unwrap();
        circuits.connect(junction_a, junction_b);
    }

    circuits.circuit_sizes().iter().rev().take(3).product()
}

fn part2(input: &str) -> i64 {
    let mut circuits = Circuits::from_str(input).unwrap();

    loop {
        let (_, junction_a, junction_b) = circuits.distances.pop().unwrap();
        circuits.connect(junction_a, junction_b);

        if circuits.circuits.len() == 1 {
            return junction_a.0 * junction_b.0;
        }
    }
}

type Junction = (i64, i64, i64);

struct Circuits {
    circuits: Vec<Circuit>,
    distances: Vec<(f64, Junction, Junction)>,
}

#[derive(Clone)]
struct Circuit {
    junctions: HashSet<Junction>,
}

impl Circuits {
    fn connect(&mut self, junction_a: Junction, junction_b: Junction) {
        let (circuit_b_index, circuit_b) = self
            .circuits
            .iter()
            .enumerate()
            .find_map(|(index, c)| {
                if c.junctions.contains(&junction_b) {
                    Some((index, c.clone()))
                } else {
                    None
                }
            })
            .unwrap();

        let circuit_a = self
            .circuits
            .iter_mut()
            .find(|c| c.junctions.contains(&junction_a))
            .unwrap();

        // if both are already in the same circuit, nothing happens
        if circuit_a.junctions.contains(&junction_b) {
            return;
        }

        // merge circuits
        for junction in circuit_b.junctions {
            circuit_a.junctions.insert(junction);
        }

        self.circuits.remove(circuit_b_index);
    }

    fn circuit_sizes(&self) -> Vec<usize> {
        let mut circuit_sizes = self
            .circuits
            .iter()
            .map(|circuit| circuit.junctions.len())
            .collect::<Vec<_>>();

        circuit_sizes.sort();

        circuit_sizes
    }
}

impl FromStr for Circuits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let junctions = s
            .lines()
            .map(|line| {
                let axes = line
                    .split(',')
                    .map(|axis| axis.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();

                (
                    *axes.get(0).unwrap(),
                    *axes.get(1).unwrap(),
                    *axes.get(2).unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let mut distances = junctions
            .iter()
            .enumerate()
            .flat_map(|(i, junction)| {
                junctions.iter().skip(i + 1).map(|other_junction| {
                    let distance = (((junction.0 - other_junction.0).pow(2)
                        + (junction.1 - other_junction.1).pow(2)
                        + (junction.2 - other_junction.2).pow(2))
                        as f64)
                        .sqrt();

                    (distance, *junction, *other_junction)
                })
            })
            .collect::<Vec<_>>();
        distances.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let circuits = junctions
            .iter()
            .map(|junction| Circuit {
                junctions: HashSet::from_iter(vec![*junction]),
            })
            .collect::<Vec<Circuit>>();

        Ok(Self {
            circuits,
            distances,
        })
    }
}
