use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn get_visit_count(nodes: &[Vec<usize>]) -> usize {
    let mut visited = vec![false; nodes.len()];
    let start = 0;

    let mut visited_count = 0;

    let mut queue = VecDeque::from([start]);
    while let Some(current_node) = queue.pop_front() {
        if visited[current_node] {
            continue;
        }
        visited_count += 1;
        visited[current_node] = true;
        for &child in &nodes[current_node] {
            queue.push_back(child);
        }
    }

    visited_count
}

fn part1(nodes: &Vec<Vec<usize>>) -> usize {
    let mut nodes = nodes.clone();
    for _ in 0..3 {
        let (left, right) = get_strongest_link(&nodes);
        nodes[left].retain(|&n| n != right);
        nodes[right].retain(|&n| n != left);
    }
    let size = get_visit_count(&nodes);
    size * (nodes.len() - size)
}

fn get_strongest_link(nodes: &[Vec<usize>]) -> (usize, usize) {
    let mut passthrough_count: HashMap<(usize, usize), usize> = HashMap::new();

    for node in 0..nodes.len() {
        let mut queue = VecDeque::from([node]);
        let mut seen = vec![false; nodes.len()];
        seen[node] = true;

        while let Some(current_node) = queue.pop_front() {
            for &child in &nodes[current_node] {
                if seen[child] {
                    continue;
                }
                seen[child] = true;

                let min = current_node.min(child);
                let max = current_node.max(child);

                *passthrough_count.entry((min, max)).or_default() += 1;

                queue.push_back(child);
            }
        }
    }

    passthrough_count
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(link, _)| link)
        .unwrap()
}

fn parse_graph(input: &str) -> Vec<Vec<usize>> {
    let mut nodes = Vec::new();
    let mut ids = HashMap::new();

    for line in input.lines() {
        if let Some((node_name, children_str)) = line.split_once(": ") {
            let next_id = ids.len();
            let node_id = *ids.entry(node_name).or_insert(next_id);

            for children_name in children_str.split(" ") {
                let next_id = ids.len();
                let children_id = *ids.entry(children_name).or_insert(next_id);
                nodes.resize(nodes.len().max(node_id + 1).max(children_id + 1), Vec::new());
                nodes[node_id].push(children_id);
                nodes[children_id].push(node_id);
            }
        }
    }

    nodes
}

fn main() {
    let input = include_str!("input.txt");
    let graph = parse_graph(input);

    let instant = Instant::now();

    let p1 = part1(&graph);
    println!("Day 25: {} in {:?}", p1, instant.elapsed());
}
