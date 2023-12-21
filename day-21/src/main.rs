use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq)]
enum Tile {
    GardenPlot,
    Rock,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn apply_to_position(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Direction::North => (x, y.checked_sub(1)?),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x.checked_sub(1)?, y),
        })
    }

    fn apply_to_position_with_margins(&self, (x, y): (usize, usize), (width, height): (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = self.apply_to_position((x, y))?;

        if x >= width || y >= height {
            None
        } else {
            Some((x, y))
        }
    }
}

struct Map {
    starting_position: (usize, usize),
    grid: Vec<Vec<Tile>>,
}

impl Map {
    fn width(&self) -> usize {
        self.grid.get(0).map(Vec::len).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get_all_visited(&self) -> HashMap<(usize, usize), usize> {
        let current = self.starting_position.clone();

        let mut visited = HashMap::new();

        let mut stack = VecDeque::new();
        stack.push_back((current, 0));

        let map_size = (self.width(), self.height());

        while let Some((position, steps)) = stack.pop_front() {
            if visited.contains_key(&position) {
                continue;
            }

            visited.insert(position, steps);

            for direction in [Direction::North, Direction::South, Direction::West, Direction::East] {
                if let Some((new_x, new_y)) = direction.apply_to_position_with_margins(position, map_size) {
                    if self.grid[new_y][new_x] == Tile::GardenPlot {
                        stack.push_back(((new_x, new_y), steps + 1))
                    }
                }
            }
        }

        visited
    }
}

fn parse(input: &str) -> Map {
    let mut starting_position = (0, 0);
    let grid = input.lines().enumerate().map(|(y, line)|
        line.chars().enumerate().map(|(x, char)| match char {
            '.' => Tile::GardenPlot,
            '#' => Tile::Rock,
            'S' => {
                starting_position = (x, y);
                Tile::GardenPlot
            }
            _ => panic!("invalid char input"),
        }).collect()).collect();

    Map {
        starting_position,
        grid,
    }
}

// Based on https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn part2(map: &Map) -> usize {
    let visited = map.get_all_visited();

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    // This is 202300 but im writing it out here to show the process
    let n = (26501365 - (map.width() / 2)) / map.width();
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    odd * visited.values().filter(|v| **v % 2 == 1).count()
        + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners)
}

fn main() {
    let input = include_str!("input.txt");
    let map = parse(input);

    let part1 = map.get_all_visited()
        .values()
        .filter(|v| **v <= 64 && **v % 2 == 0)
        .count();

    println!("Part 1: {}", part1);

    let part2 = part2(&map);
    println!("Part 2: {}", part2);
}
