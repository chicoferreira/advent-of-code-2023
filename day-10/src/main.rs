use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn apply(&self, current: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (current.0, current.1 - 1),
            Direction::South => (current.0, current.1 + 1),
            Direction::East => (current.0 + 1, current.1),
            Direction::West => (current.0 - 1, current.1),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Tile {
    fn get_connections(&self) -> (Direction, Direction) {
        match self {
            Tile::Vertical => (Direction::North, Direction::South),
            Tile::Horizontal => (Direction::East, Direction::West),
            Tile::NorthEast => (Direction::North, Direction::East),
            Tile::NorthWest => (Direction::North, Direction::West),
            Tile::SouthWest => (Direction::South, Direction::West),
            Tile::SouthEast => (Direction::South, Direction::East),
        }
    }

    fn get_next_direction(&self, coming_from: Direction) -> Option<Direction> {
        let (first, second) = self.get_connections();

        if coming_from == first {
            Some(second)
        } else if coming_from == second {
            Some(first)
        } else {
            None
        }
    }

    fn has_connection(&self, direction: Direction) -> bool {
        self.get_next_direction(direction).is_some()
    }

    fn is_connection_up(&self) -> bool {
        self.has_connection(Direction::North)
    }
}

struct Grid {
    start: (usize, usize),
    tiles: Vec<Vec<Option<Tile>>>,
}

impl Grid {
    fn get_x_size(&self) -> usize {
        self.tiles[0].len()
    }

    fn get_y_size(&self) -> usize {
        self.tiles.len()
    }

    fn get_tile(&self, position: &(usize, usize)) -> Option<&Tile> {
        let x = self.tiles.get(position.1)?;
        let y = x.get(position.0)?;
        y.as_ref()
    }

    fn is_tile_up(&self, position: &(usize, usize)) -> bool {
        position == &self.start || self.get_tile(position).map(|tile| tile.is_connection_up()).unwrap_or(false)
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

fn get_loop(grid: &Grid) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let start = grid.start;

    let mut path = DIRECTIONS.into_iter()
        .map(|direction| (direction.clone(), direction.apply(&start)))  // assume start is not at the border (otherwise overflow would happen)
        .filter(|(dir, pos)| grid.get_tile(pos).map(|tile| tile.has_connection(dir.opposite())).unwrap_or(false))
        .last()
        .expect("start has to have at least one connection");

    result.push(start);

    while path.1 != start {
        let last_direction = &mut path.0;
        let current_pos = &mut path.1;

        let pipe = grid.get_tile(current_pos).expect("this always has to be a pipe");

        let next_direction = pipe
            .get_next_direction(last_direction.opposite())
            .expect("this always has to be a direction");

        result.push(*current_pos);

        *current_pos = next_direction.apply(current_pos);
        *last_direction = next_direction;
    }

    result
}

fn get_longest_path(grid: &Grid) -> usize {
    get_loop(grid).len() / 2
}

fn get_enclosed_by_the_loop(grid: &Grid) -> usize {
    let mut inside = Vec::new();

    let grid_loop: HashSet<(usize, usize)> = HashSet::from_iter(get_loop(grid));

    let mut result = 0;

    for y in 0..grid.get_y_size() {
        let mut is_inside = false;
        for x in 0..grid.get_x_size() {
            let position = (x, y);
            if grid_loop.contains(&position) && grid.is_tile_up(&position) {
                is_inside = !is_inside;
            } else if is_inside && !grid_loop.contains(&position) {
                result += 1;
                inside.push(position);
            }
        }
    }

    result
}

mod parser {
    use crate::{Grid, Tile};

    pub(crate) fn parse_input(input: &str) -> Grid {
        let mut start = None;

        let tiles = input.lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        match c {
                            '|' => Some(Tile::Vertical),
                            '-' => Some(Tile::Horizontal),
                            'L' => Some(Tile::NorthEast),
                            'J' => Some(Tile::NorthWest),
                            '7' => Some(Tile::SouthWest),
                            'F' => Some(Tile::SouthEast),
                            '.' => None,
                            'S' => {
                                start = Some((x, y));
                                None
                            }
                            _ => panic!("invalid character"),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Grid { start: start.expect("start not found"), tiles }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let grid = parser::parse_input(input);

    println!("Part 1: {:?}", get_longest_path(&grid));
    println!("Part 2: {:?}", get_enclosed_by_the_loop(&grid));
}
