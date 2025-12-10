use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Machine::from_str(line).unwrap().solve())
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Machine::from_str(line).unwrap().solve_power_levels())
        .sum()
}

#[derive(Debug)]
struct Machine {
    target_state: MachineState,
    buttons: Vec<Vec<usize>>,
    target_power_state: Vec<u64>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct MachineState(usize);

impl Machine {
    fn solve(&self) -> usize {
        let mut states = HashMap::new();

        let mut heap = BinaryHeap::new();
        heap.push(DijkstraState {
            machine_state: MachineState(0),
            button_presses: 0,
        });

        while let Some(DijkstraState {
            machine_state,
            button_presses,
        }) = heap.pop()
        {
            if machine_state.0 == self.target_state.0 {
                return button_presses;
            }

            if states
                .get(&machine_state.0)
                .is_some_and(|min_button_presses| button_presses > *min_button_presses)
            {
                continue;
            }

            for button in &self.buttons {
                let next_state = DijkstraState {
                    machine_state: machine_state.after_button_press(button),
                    button_presses: button_presses + 1,
                };

                if next_state.button_presses
                    < *states
                        .get(&next_state.machine_state.0)
                        .unwrap_or(&usize::MAX)
                {
                    heap.push(next_state);
                    states.insert(next_state.machine_state.0, next_state.button_presses);
                }
            }
        }

        usize::MAX
    }

    fn solve_power_levels(&self) -> u64 {
        let mut vars = variables!();
        let button_press_counts = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect::<Vec<_>>();

        let objective: Expression = button_press_counts.iter().sum();

        let mut model = vars.minimise(objective).using(default_solver);

        for (i, power_level) in self.target_power_state.iter().enumerate() {
            let mut power_level_expression: Expression = 0.into();

            for (button_index, button) in self.buttons.iter().enumerate() {
                if button.contains(&i) {
                    power_level_expression += button_press_counts[button_index];
                }
            }

            model.add_constraint(constraint!(power_level_expression == *power_level as f64));
        }

        let solution = model.solve().unwrap();

        button_press_counts
            .iter()
            .map(|var| solution.value(*var).round() as u64)
            .sum()
    }
}

impl MachineState {
    fn after_button_press(&self, button: &Vec<usize>) -> MachineState {
        MachineState(self.0 ^ MachineState::from_positions(button).0)
    }

    fn from_positions(positions: &Vec<usize>) -> Self {
        MachineState(positions.iter().fold(0, |state, pos| state | (1 << *pos)))
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = s.split(" ").collect::<Vec<_>>();
        let first_group = groups[0];
        let last_group = groups[groups.len() - 1];

        let target_state = MachineState::from_positions(
            &first_group[1..first_group.len() - 1]
                .chars()
                .enumerate()
                .filter_map(|(pos, c)| if c == '#' { Some(pos) } else { None })
                .collect::<Vec<_>>(),
        );

        let buttons = groups
            .iter()
            .skip(1)
            .take(groups.len() - 2)
            .map(|group| {
                group[1..group.len() - 1]
                    .split(",")
                    .map(|pos| pos.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let target_power_state = last_group[1..last_group.len() - 1]
            .split(",")
            .map(|pos| pos.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            target_state,
            buttons,
            target_power_state,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct DijkstraState {
    machine_state: MachineState,
    button_presses: usize,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.button_presses.cmp(&self.button_presses)
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
