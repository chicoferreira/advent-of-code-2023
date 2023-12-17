use std::collections::HashSet;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    /// Mirror: /
    Mirror,
    // InvertedMirror: \
    InvertedMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East, Direction::West, Direction::South];

impl Direction {
    fn apply_to_position(&self, (x, y): (usize, usize), (width, height): (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = match self {
            Direction::North => (x, y.checked_sub(1)?),
            Direction::East => (x + 1, y),
            Direction::West => (x.checked_sub(1)?, y),
            Direction::South => (x, y + 1),
        };

        if x >= width || y >= height {
            None
        } else {
            Some((x, y))
        }
    }

    fn reflect(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::South => Direction::West,
        }
    }

    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
        }
    }

    fn inverted_reflect(self) -> Self {
        self.reflect().opposite()
    }
}

impl Tile {
    fn beam_encounter_result(&self, from_direction: Direction) -> Vec<Direction> {
        match self {
            Tile::Empty => vec![from_direction],
            Tile::Mirror => vec![from_direction.reflect()],
            Tile::InvertedMirror => vec![from_direction.inverted_reflect()],
            Tile::VerticalSplitter => match from_direction {
                Direction::North | Direction::South => vec![from_direction],
                Direction::East | Direction::West => vec![Direction::North, Direction::South],
            },
            Tile::HorizontalSplitter => match from_direction {
                Direction::North | Direction::South => vec![Direction::West, Direction::East],
                Direction::East | Direction::West => vec![from_direction],
            }
        }
    }
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.grid.get(0).map(Vec::len).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        self.grid.get(y)?.get(x).map(|t| *t)
    }
}

fn trace_light(grid: &Grid, start_position: (usize, usize), start_direction: Direction) -> u64 {
    let grid_size = (grid.width(), grid.height());

    let mut visited_with_direction = HashSet::new();
    let mut visited = HashSet::new();

    let mut result = 0;
    let mut beams = vec![(start_position, start_direction)];

    while let Some(((x, y), previous_direction)) = beams.pop() {
        visited_with_direction.insert((previous_direction.clone(), (x, y)));
        if visited.insert((x, y)) {
            result += 1;
        }

        let tile = grid.get_tile(x, y).expect("tile has to be in grid");

        tile.beam_encounter_result(previous_direction).into_iter()
            .filter_map(|d| Some((d.apply_to_position((x, y).clone(), grid_size)?, d)))
            .filter(|(p, d)| !visited_with_direction.contains(&(*d, *p)))
            .for_each(|(p, d)| beams.push((p, d)));
    }

    result
}

fn parse_grid(input: &str) -> Grid {
    let grid = input.lines().map(
        |line| line.chars()
            .map(|c| match c {
                '.' => Tile::Empty,
                '/' => Tile::Mirror,
                '\\' => Tile::InvertedMirror,
                '-' => Tile::HorizontalSplitter,
                '|' => Tile::VerticalSplitter,
                c => panic!("unknown char: {}", c)
            }).collect()
    ).collect();

    Grid {
        grid
    }
}

fn part1(grid: &Grid) -> u64 {
    trace_light(grid, (0, 0), Direction::East)
}

fn part2(grid: &Grid) -> u64 {
    (0..grid.width()).into_iter()
        .flat_map(|x| (0..grid.height()).into_iter().map(move |y| (x, y)))
        .filter(|&(x, y)| x == 0 || y == 0 || x == grid.width() - 1 || y == grid.height() - 1)
        .flat_map(|position| DIRECTIONS.into_iter().map(move |d| (position, d.clone())))
        .map(|(start_position, start_direction)| trace_light(grid, start_position, start_direction))
        .max()
        .expect("map shouldn't be empty")
}

fn main() {
    let input = include_str!("input.txt");
    let grid = parse_grid(input);

    let instant = Instant::now();
    let part1 = part1(&grid);
    println!("Part 1 in {:?}: {}", instant.elapsed(), part1);

    let instant = Instant::now();
    let part2 = part2(&grid);
    println!("Part 2 in {:?}: {}", instant.elapsed(), part2);
}
