use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

struct Map {
    grid: Vec<Vec<u8>>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply_to_position(&self, (x, y): (usize, usize), (width, height): (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = match self {
            Direction::Up => (x, y.checked_sub(1)?),
            Direction::Right => (x + 1, y),
            Direction::Left => (x.checked_sub(1)?, y),
            Direction::Down => (x, y + 1),
        };

        if x >= width || y >= height {
            None
        } else {
            Some((x, y))
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }
}

impl Map {
    fn width(&self) -> usize {
        self.grid.get(0).map(Vec::len).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

#[derive(Eq, PartialEq)]
struct Path(u64, u64, (usize, usize), Direction);

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_min_heat_loss(map: &Map, min_same_direction: u64, max_same_direction: u64) -> u64 {
    let mut min_heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    min_heap.push(Path(map.grid[0][1] as u64, 1, (1, 0), Direction::Right));
    min_heap.push(Path(map.grid[1][0] as u64, 1, (0, 1), Direction::Down));

    while let Some(Path(accumulated_heat_loss,
                        consecutive_moves_in_same_direction,
                        (x, y),
                        last_direction)) = min_heap.pop() {
        if x == map.width() - 1 && y == map.height() - 1 && consecutive_moves_in_same_direction >= min_same_direction {
            return accumulated_heat_loss;
        }

        for direction in &[Direction::Up, Direction::Right, Direction::Left, Direction::Down] {
            if *direction == last_direction.opposite() {
                continue;
            }

            if *direction != last_direction && consecutive_moves_in_same_direction < min_same_direction {
                continue;
            }

            let Some((x, y)) = direction.apply_to_position((x, y), (map.width(), map.height())) else {
                continue;
            };

            let accumulated_heat_loss = accumulated_heat_loss + map.grid[y][x] as u64;

            let consecutive_moves_in_same_direction = if *direction == last_direction {
                consecutive_moves_in_same_direction + 1
            } else {
                1
            };

            if consecutive_moves_in_same_direction > max_same_direction {
                continue;
            }

            if seen.insert(((x, y), *direction, consecutive_moves_in_same_direction)) {
                min_heap.push(Path(accumulated_heat_loss,
                                   consecutive_moves_in_same_direction,
                                   (x, y),
                                   *direction));
            }
        }
    }

    u64::MAX
}

fn parse_map(input: &str) -> Map {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("input only contains numbers") as u8)
                .collect()
        })
        .collect();

    Map { grid }
}

fn main() {
    let input = include_str!("input.txt");
    let map = parse_map(input);

    let instant = Instant::now();
    let part1 = get_min_heat_loss(&map, 1, 3);
    println!("Part 1 in {:?}: {}", instant.elapsed(), part1);

    let instant = Instant::now();
    let part2 = get_min_heat_loss(&map, 4, 10);
    println!("Part 2 in {:?}: {}", instant.elapsed(), part2);
}
