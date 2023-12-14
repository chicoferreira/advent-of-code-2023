use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    RoundedRock,
    CubedRock,
    Empty,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::RoundedRock => write!(f, "O"),
            Tile::CubedRock => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{:?}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn get_tile(&self, pos: (usize, usize)) -> Option<Tile> {
        let (x, y) = pos;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.tiles[y][x])
    }

    fn is_tile(&self, pos: (usize, usize), tile: Tile) -> bool {
        self.get_tile(pos) == Some(tile)
    }
}

// Only works for part 1
fn get_total_north_roll_load(map: &Map) -> usize {
    let mut total_load = 0;

    for x in 0..map.width {
        let mut start_of_rounded_rock = map.height;
        let mut rounded_rock_count = 0;
        for y in 0..map.height {
            match map.tiles[y][x] {
                Tile::RoundedRock => rounded_rock_count += 1,
                Tile::CubedRock => {
                    if rounded_rock_count > 0 {
                        total_load += get_sum_between(start_of_rounded_rock - rounded_rock_count + 1, start_of_rounded_rock);
                        rounded_rock_count = 0;
                    }
                    start_of_rounded_rock = map.height - y - 1;
                }
                Tile::Empty => {}
            }
        }
        if rounded_rock_count > 0 {
            total_load += get_sum_between(start_of_rounded_rock - rounded_rock_count + 1, start_of_rounded_rock);
        }
    }

    total_load
}

fn get_sum_between(start: usize, end: usize) -> usize {
    let n = end - start + 1;

    (start + end) * n / 2
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn apply_offset(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        Some(match self {
            Direction::North => (x, y.checked_sub(1)?),
            Direction::West => (x.checked_sub(1)?, y),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
        })
    }
}

fn cycle(map: &mut Map) {
    roll_map(map, Direction::North);
    roll_map(map, Direction::West);
    roll_map(map, Direction::South);
    roll_map(map, Direction::East);
}

fn roll_map(map: &mut Map, direction: Direction) {
    for x in 0..map.width {
        for y in 0..map.height {
            let mut current = (x, y);

            let mut target = current;

            while map.get_tile(current) == Some(Tile::Empty) ||
                map.get_tile(current) == Some(Tile::RoundedRock) {
                let option = direction.apply_offset(current);
                if let Some(option) = option {
                    current = option;
                } else {
                    break;
                }

                if map.get_tile(current) == Some(Tile::Empty) {
                    target = current;
                }
            }

            if target != (x, y) {
                let tile = map.tiles[y][x];
                map.tiles[y][x] = map.tiles[target.1][target.0];
                map.tiles[target.1][target.0] = tile;
            }
        }
    }
}

fn parse_input(input: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = input.lines().map(|line| line.chars().map(|c| match c {
        'O' => Tile::RoundedRock,
        '.' => Tile::Empty,
        '#' => Tile::CubedRock,
        _ => panic!("Unknown tile: {}", c),
    }).collect()).collect();

    let width = tiles.get(0).map(|row| row.len()).unwrap_or(0);
    let height = tiles.len();

    Map { tiles, width, height }
}

fn get_total_north_load(map: &Map) -> usize {
    let mut answer = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if map.tiles[y][x] == Tile::RoundedRock {
                answer += map.height - y;
            }
        }
    }

    answer
}

fn part2(map: &Map) -> usize {
    let mut map = map.clone();

    let mut cache = HashMap::new();
    cache.insert(map.clone(), 0);

    let mut loop_len = 0;
    let mut start_of_loop = 0;

    for current_index in 0..1_000_000_000 {
        cycle(&mut map);
        if let Some(new_start_of_loop) = cache.insert(map.clone(), current_index + 1) {
            start_of_loop = new_start_of_loop;
            loop_len = current_index - new_start_of_loop + 1;
            println!("Loop found from {} to {}", start_of_loop, start_of_loop + loop_len);
            break;
        }
    }

    for _ in 0..(1_000_000_000 - start_of_loop) % loop_len {
        cycle(&mut map);
    }

    get_total_north_load(&map)
}

fn main() {
    let input = include_str!("input.txt");
    let map = parse_input(input);

    let part1 = get_total_north_roll_load(&map);
    println!("Part 1: {}", part1);

    let part2 = part2(&mut map.clone());
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_sum_between() {
        let result = super::get_sum_between(10, 10);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_get_total_north_roll_load() {
        let map = super::parse_input(r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#);
        assert_eq!(super::get_total_north_roll_load(&map), 136);
    }

    #[test]
    fn roll_north() {
        let mut map = super::parse_input(r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#);
        super::roll_map(&mut map, super::Direction::North);
        assert_eq!(map, super::parse_input(r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#));
    }
}