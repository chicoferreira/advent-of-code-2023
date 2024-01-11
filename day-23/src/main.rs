use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

struct Map {
    nodes: Vec<Vec<Tile>>,
}

impl Map {
    fn width(&self) -> usize {
        self.nodes[0].len()
    }

    fn height(&self) -> usize {
        self.nodes.len()
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.nodes.get(y)?.get(x)
    }

    fn apply(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        let (x, y) = match direction {
            Direction::North => Some((x, y.checked_sub(1)?)),
            Direction::East => Some((x + 1, y)),
            Direction::West => Some((x.checked_sub(1)?, y)),
            Direction::South => Some((x, y + 1)),
        }?;

        self.get(x, y).filter(|tile| tile != &&Tile::Forest).map(|_| (x, y))
    }

    fn get_possible_next_part1(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match &self.nodes[y][x] {
            Tile::Path => vec![Direction::North, Direction::East, Direction::West, Direction::South],
            Tile::Forest => vec![],
            Tile::Slope(direction) => vec![*direction],
        }.iter().filter_map(|direction| self.apply(x, y, *direction)).collect()
    }

    fn get_possible_next_part2(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match &self.nodes[y][x] {
            Tile::Forest => vec![],
            Tile::Path | Tile::Slope(_) => vec![Direction::North, Direction::East, Direction::West, Direction::South],
        }.iter().filter_map(|direction| self.apply(x, y, *direction)).collect()
    }
}

type NeighborsFn = fn(map: &Map, usize, usize) -> Vec<(usize, usize)>;
type Position = (usize, usize);

fn contract_map(map: &Map, possible_next_fn: NeighborsFn) -> HashMap<Position, Vec<(Position, usize)>> {
    let mut nodes = HashMap::new();

    fn find_edge_intersections(map: &Map, possible_next_fn: NeighborsFn, pos: Position) -> Vec<(Position, usize)> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        visited.insert(pos);

        let start = (1, 0);
        let goal = (map.width() - 2, map.height() - 1);

        let mut queue = vec![(pos, 0)];

        while let Some((current, steps)) = queue.pop() {
            let neighbors = possible_next_fn(map, current.0, current.1);
            if (neighbors.len() > 2 || current == goal || current == start) && current != pos {
                result.push((current, steps));
                continue;
            }

            for next in neighbors {
                if visited.contains(&next) {
                    continue;
                }

                visited.insert(next);
                queue.push((next, steps + 1));
            }
        }

        result
    }

    let start = (1, 0);
    let goal = (map.width() - 2, map.height() - 1);

    for y in 0..map.height() {
        for x in 0..map.width() {
            let pos = (x, y);
            let neighbors = possible_next_fn(map, x, y);

            if neighbors.len() > 2 || pos == start || pos == goal {
                nodes.insert(pos, find_edge_intersections(map, possible_next_fn, pos));
            }
        }
    }

    nodes
}

fn dfs(pos: Position, goal: Position, contracted_grid: &HashMap<Position, Vec<(Position, usize)>>, seen: &mut HashSet<Position>) -> usize {
    if pos == goal {
        return 0;
    }
    seen.insert(pos);

    let mut max = isize::MIN;
    for (next_position, cost) in contracted_grid.get(&pos).expect("all neighbors should be in the contracted grid") {
        if !seen.contains(next_position) {
            let v = dfs(*next_position, goal, contracted_grid, seen) + cost;
            max = max.max(v as isize);
        }
    }

    seen.remove(&pos);

    max as usize
}

fn get_steps_of_longest_hike(map: &Map, possible_next_fn: NeighborsFn) -> usize {
    let contracted_map = contract_map(map, possible_next_fn);

    dfs((1, 0), (map.width() - 2, map.height() - 1), &contracted_map, &mut HashSet::new())
}

fn part1(map: &Map) -> usize {
    get_steps_of_longest_hike(map, Map::get_possible_next_part1)
}

fn part2(map: &Map) -> usize {
    get_steps_of_longest_hike(map, Map::get_possible_next_part2)
}

fn parse_map(input: &str) -> Map {
    let nodes = input.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '>' => Tile::Slope(Direction::East),
            'v' => Tile::Slope(Direction::South),
            _ => panic!("Unknown tile {c}"),
        }).collect()
    }).collect();

    Map { nodes }
}

fn main() {
    let input = include_str!("input.txt");
    let map = parse_map(input);

    let instant = Instant::now();
    let part1 = part1(&map);
    println!("Part 1: {part1} in {:?}", instant.elapsed());

    let instant = Instant::now();
    let part2 = part2(&map);
    println!("Part 2: {part2} in {:?}", instant.elapsed());
}