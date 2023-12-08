use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
enum Direction { Left, Right }

struct Node {
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn go_to(&self, direction: &Direction) -> &NodeId {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct NodeId([char; 3]);

impl NodeId {
    fn get_ending_character(&self) -> char {
        self.0[2]
    }
}

impl FromStr for NodeId {
    type Err = Vec<char>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeId(s.chars().collect::<Vec<char>>().try_into()?))
    }
}

struct Network {
    nodes: HashMap<NodeId, Node>,
}

impl Network {
    fn get(&self, node_id: &NodeId) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    fn get_ghost_nodes(&self, ending_character: char) -> Vec<&NodeId> {
        self.nodes.keys()
            .filter(|id| id.get_ending_character() == ending_character)
            .collect()
    }
}


fn part1(network: &Network, instructions: &Vec<Direction>) -> u64 {
    let start: NodeId = "AAA".parse().unwrap();
    let destination: NodeId = "ZZZ".parse().unwrap();

    let mut current_node_id = &start;
    let mut cycle = instructions.iter().cycle();

    let mut steps = 0;

    while current_node_id != &destination {
        let next_direction = cycle.next().expect("cycle should never end");
        current_node_id = network.get(current_node_id).expect("every destination should be a node").go_to(next_direction);
        steps += 1;
    }

    steps
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn part2(network: &Network, instructions: &Vec<Direction>) -> u64 {
    network.get_ghost_nodes('A').into_iter()
        .map(|current_node| {
            let mut current_node_id = current_node;
            let mut cycle = instructions.iter().cycle();

            let mut steps = 0;

            while current_node_id.get_ending_character() != 'Z' {
                let next_direction = cycle.next().expect("cycle should never end");
                current_node_id = network.get(current_node_id).expect("every destination should be a node").go_to(next_direction);
                steps += 1;
            }

            steps
        })
        .reduce(lcm)
        .expect("there should be at least one node")
}

mod parser {
    use std::collections::HashMap;

    use crate::{Direction, Network, Node, NodeId};

    fn parse_directions(input: &str) -> Vec<Direction> {
        input.chars().map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        }).collect()
    }

    fn parse_node_line(input: &str) -> Option<(NodeId, Node)> {
        let (start, destinations) = input.split_once(" = ")?;
        let start = start.parse().ok()?;

        let (left, right) = destinations.trim_start_matches('(').trim_end_matches(')').split_once(", ")?;
        let left = left.parse().ok()?;
        let right = right.parse().ok()?;

        Some((start, Node { left, right }))
    }

    pub(crate) fn parse_input(input: &str) -> (Vec<Direction>, Network) {
        let mut lines = input.lines();
        let directions = parse_directions(lines.next().expect("input should have at least one line"));

        lines.next(); // skip empty line

        let nodes = lines.filter_map(parse_node_line).collect::<HashMap<NodeId, Node>>();

        (directions, Network { nodes })
    }
}

fn main() {
    let input = include_str!("input.txt");

    let (directions, network) = parser::parse_input(input);

    println!("Part 1: {}", part1(&network, &directions));
    println!("Part 2: {}", part2(&network, &directions));
}
