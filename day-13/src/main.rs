#[derive(Eq, PartialEq, Debug)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
enum MirrorDirection {
    Horizontal,
    Vertical,
}

impl Map {
    fn width(&self) -> usize {
        self.tiles.get(0).map(Vec::len).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn match_columns(&self, column1: usize, column2: usize) -> bool {
        for y in 0..self.height() {
            if self.tiles[y][column1] != self.tiles[y][column2] {
                return false;
            }
        }
        true
    }

    fn match_rows(&self, row1: usize, row2: usize) -> bool {
        self.tiles[row1] == self.tiles[row2]
    }

    fn generic_get_mirror_position(&self, lines_match: fn(&Map, usize, usize) -> bool, size_to_loop: usize) -> Option<usize> {
        'outer: for current in 1..size_to_loop {
            let previous_line = current - 1;

            if lines_match(self, previous_line, current) {
                let reflection_pos = previous_line;

                for current_rest in current + 1..size_to_loop {
                    if reflection_pos * 2 + 1 < current_rest {
                        continue;
                    }

                    if !lines_match(self, current_rest, reflection_pos * 2 + 1 - current_rest) {
                        continue 'outer;
                    }
                }

                return Some(reflection_pos + 1);
            }
        }
        None
    }

    fn get_vertical_mirror_position(&self) -> Option<usize> {
        self.generic_get_mirror_position(Map::match_columns, self.width())
    }

    fn get_horizontal_mirror_position(&self) -> Option<usize> {
        self.generic_get_mirror_position(Map::match_rows, self.height())
    }

    fn get_columns_difference(&self, column1: usize, column2: usize) -> usize {
        let mut difference = 0;
        for y in 0..self.height() {
            if self.tiles[y][column1] != self.tiles[y][column2] {
                difference += 1;
            }
        }
        difference
    }

    fn get_rows_difference(&self, row1: usize, row2: usize) -> usize {
        let row1 = &self.tiles[row1];
        let row2 = &self.tiles[row2];

        row1.iter()
            .zip(row2.iter())
            .filter(|(tile1, tile2)| tile1 != tile2)
            .count()
    }

    fn generic_get_mirror_position_with_differences(&self, difference_match: fn(&Map, usize, usize) -> usize, size_to_loop: usize) -> Option<usize> {
        'outer: for current in 1..size_to_loop {
            let previous_line = current - 1;

            let difference = difference_match(self, previous_line, current);
            let mut has_changed = difference > 0;
            if difference <= 1 {
                let reflection_pos = previous_line;

                for current_rest in current + 1..size_to_loop {
                    if reflection_pos * 2 + 1 < current_rest {
                        continue;
                    }

                    let i = difference_match(self, current_rest, reflection_pos * 2 + 1 - current_rest);

                    if i > 1 || i == 1 && has_changed {
                        continue 'outer;
                    }

                    has_changed = has_changed || i > 0;
                }

                if has_changed {
                    return Some(reflection_pos + 1);
                }
            }
        }
        None
    }

    fn get_mirror_position(&self) -> (MirrorDirection, usize) {
        if let Some(position) = self.get_vertical_mirror_position() {
            return (MirrorDirection::Vertical, position);
        }
        if let Some(position) = self.get_horizontal_mirror_position() {
            return (MirrorDirection::Horizontal, position);
        }
        panic!("No mirror found");
    }

    fn get_mirror_position_with_differences(&self) -> (MirrorDirection, usize) {
        if let Some(position) = self.get_vertical_mirror_with_smudge_position() {
            return (MirrorDirection::Vertical, position);
        }
        if let Some(position) = self.get_horizontal_mirror_with_smudge_position() {
            return (MirrorDirection::Horizontal, position);
        }
        panic!("No mirror found");
    }

    fn get_vertical_mirror_with_smudge_position(&self) -> Option<usize> {
        self.generic_get_mirror_position_with_differences(Map::get_columns_difference, self.width())
    }

    fn get_horizontal_mirror_with_smudge_position(&self) -> Option<usize> {
        self.generic_get_mirror_position_with_differences(Map::get_rows_difference, self.height())
    }
}


fn main() {
    let input = include_str!("input.txt");
    let maps = parse(input);

    println!("Part 1: {:?}", part1(&maps));
    println!("Part 2: {:?}", part2(&maps));
}

fn part1(maps: &[Map]) -> usize {
    maps.iter().map(|map| map.get_mirror_position())
        .map(|(direction, position)| {
            match direction {
                MirrorDirection::Horizontal => 100 * position,
                MirrorDirection::Vertical => position,
            }
        }).sum()
}

fn part2(maps: &[Map]) -> usize {
    maps.iter().map(|map| map.get_mirror_position_with_differences())
        .map(|(direction, position)| {
            match direction {
                MirrorDirection::Horizontal => 100 * position,
                MirrorDirection::Vertical => position,
            }
        }).sum()
}

fn parse_map(input: &str) -> Map {
    let tiles = input.lines().map(|line| {
        line.chars().filter(|c| !c.is_whitespace()).map(|character| match character {
            '#' => Tile::Rock,
            '.' => Tile::Ash,
            _ => panic!("Unknown tile"),
        }).collect()
    }).collect();

    Map { tiles }
}

fn parse(input: &str) -> Vec<Map> {
    input.split("\r\n\r\n").map(|map| parse_map(map)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{Map, parse_map, part1, part2};

    #[test]
    fn test_vertical_reflection() {
        let input =
            r#"#.##..##.
               ..#.##.#.
               ##......#
               ##......#
               ..#.##.#.
               ..##..##.
               #.#.##.#."#;

        let map = parse_map(input);

        assert_eq!(map.get_vertical_mirror_position(), Some(5));
    }

    #[test]
    fn test_horizontal_reflection() {
        let input =
            r#"#...##..#
               #....#..#
               ..##..###
               #####.##.
               #####.##.
               ..##..###
               #....#..#"#;

        let map = parse_map(input);

        assert_eq!(map.get_horizontal_mirror_position(), Some(4));
        assert_eq!(map.get_vertical_mirror_position(), None);
        assert_eq!(map.get_horizontal_mirror_with_smudge_position(), Some(1));
    }

    #[test]
    fn test_full_example() {
        let input = vec!(
            r#"#.##..##.
               ..#.##.#.
               ##......#
               ##......#
               ..#.##.#.
               ..##..##.
               #.#.##.#."#,
            r#"#...##..#
               #....#..#
               ..##..###
               #####.##.
               #####.##.
               ..##..###
               #....#..#"#);

        let maps = input.iter().map(|map| parse_map(map)).collect::<Vec<Map>>();

        assert_eq!(part1(&maps), 405);
    }

    #[test]
    fn test_full_with_smudge_example() {
        let input = vec!(
            r#"#.##..##.
               ..#.##.#.
               ##......#
               ##......#
               ..#.##.#.
               ..##..##.
               #.#.##.#."#,
            r#"#...##..#
               #....#..#
               ..##..###
               #####.##.
               #####.##.
               ..##..###
               #....#..#"#);

        let maps = input.iter().map(|map| parse_map(map)).collect::<Vec<Map>>();

        assert_eq!(part2(&maps), 400);
    }
}

