use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    Graph::from_str(input)
        .unwrap()
        .find_all_paths_on_route(vec![parse_device_name("you"), parse_device_name("out")])
}

fn part2(input: &str) -> usize {
    let graph = Graph::from_str(input).unwrap();

    graph.find_all_paths_on_route(vec![
        parse_device_name("svr"),
        parse_device_name("dac"),
        parse_device_name("fft"),
        parse_device_name("out"),
    ]) + graph.find_all_paths_on_route(vec![
        parse_device_name("svr"),
        parse_device_name("fft"),
        parse_device_name("dac"),
        parse_device_name("out"),
    ])
}

struct Graph {
    nodes: HashMap<DeviceName, Vec<DeviceName>>,
}

impl Graph {
    fn find_all_paths_on_route(&self, route: Vec<DeviceName>) -> usize {
        route
            .iter()
            .zip(route.iter().skip(1))
            .fold(1, |acc, (from, to)| {
                acc * self.find_all_paths(*from, *to, &mut HashMap::new())
            })
    }

    fn find_all_paths(
        &self,
        from: DeviceName,
        to: DeviceName,
        cache: &mut HashMap<DeviceName, usize>,
    ) -> usize {
        self.nodes
            .get(&from)
            .unwrap_or(&vec![])
            .iter()
            .fold(0, |paths, output| {
                if *output == to {
                    return paths + 1;
                }

                if cache.contains_key(output) {
                    return paths + cache.get(output).unwrap();
                }

                let paths_from_output = self.find_all_paths(*output, to, cache);
                cache.insert(*output, paths_from_output);

                paths + paths_from_output
            })
    }
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let devices = s
            .lines()
            .map(|line| Device::from_str(line).unwrap())
            .collect::<Vec<_>>();

        let nodes = devices.into_iter().fold(HashMap::new(), |mut map, device| {
            map.insert(device.name, device.outputs);
            map
        });

        Ok(Graph { nodes })
    }
}

struct Device {
    name: DeviceName,
    outputs: Vec<DeviceName>,
}

type DeviceName = u32;

impl FromStr for Device {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let device_name = parse_device_name(&s[0..3]);
        let outputs = s[5..]
            .split(' ')
            .map(|name| parse_device_name(name))
            .collect();
        Ok(Device {
            name: device_name,
            outputs,
        })
    }
}

fn parse_device_name(s: &str) -> DeviceName {
    s.chars()
        .enumerate()
        .fold(0, |n, (i, c)| n | ((c as u32) << i * 8))
}
