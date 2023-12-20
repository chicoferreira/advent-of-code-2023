use std::str::FromStr;

struct Map {
    paths: Vec<(Direction, isize)>,
}

impl Map {
    fn get_interior_area(&self) -> isize {
        let mut area = 0;
        let mut lat = 0;
        for (direction, meters) in &self.paths {
            match direction {
                Direction::Up => lat -= meters,
                Direction::Down => {
                    area += meters;
                    lat += meters;
                }
                Direction::Left => area += meters * (lat + 1),
                Direction::Right => area -= meters * lat,
            };
        }
        area + 1
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn parse_map(input: &str) -> Map {
    let paths = input.lines().map(|line| {
        let [direction, meters, _] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("Invalid input");
        };
        let direction = direction.parse::<Direction>().unwrap();
        let meters = meters.parse::<isize>().unwrap();

        (direction, meters)
    }).collect();

    Map { paths }
}

fn parse_map_from_colors(input: &str) -> Map {
    let paths = input.lines().map(|line| {
        let hex = line.split_whitespace().collect::<Vec<&str>>()[2].trim_start_matches("(#").trim_end_matches(")");
        let meters = isize::from_str_radix(&hex[0..5], 16).unwrap();

        let direction = isize::from_str_radix(&hex[5..6], 16).unwrap();
        let direction = match direction {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid input"),
        };
        (direction, meters)
    }).collect();

    Map { paths }
}

fn main() {
    let input = include_str!("input.txt");
    let map = parse_map(input);
    println!("Part 1: {}", map.get_interior_area());

    let map = parse_map_from_colors(input);
    println!("Part 2: {}", map.get_interior_area());
}
