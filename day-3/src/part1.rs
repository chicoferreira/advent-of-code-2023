use std::cmp::min;

struct Grid<'a> {
    matrix: Vec<&'a str>,
}

impl<'a> Grid<'a> {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.matrix.get(y).and_then(|row| row.chars().nth(x))
    }

    fn vertical_length(&self) -> usize {
        self.matrix.len()
    }

    fn horizontal_length(&self) -> usize {
        self.matrix.get(0).map_or(0, |row| row.len())
    }

    fn is_symbol(&self, x: usize, y: usize) -> bool {
        self.get(x, y)
            .map(|c| !c.is_ascii_digit() && c != '.')
            .unwrap_or(false)
    }

    fn check_symbols_in_bounds(&self, line: usize, start_x: usize, end_x: usize) -> bool {
        fn sub_no_overflow(a: usize, b: usize) -> usize {
            if a > b {
                a - b
            } else {
                0
            }
        }

        let start_x = sub_no_overflow(start_x, 1);
        let start_y = sub_no_overflow(line, 1);

        let end_x = min(end_x + 1, self.horizontal_length());
        let end_y = min(line + 1, self.vertical_length());

        for y in start_y..=end_y {
            for x in start_x..=end_x {
                if self.is_symbol(x, y) {
                    return true;
                }
            }
        }
        return false;
    }
}

fn parse_input<'a>(input: &'a str) -> Grid<'a> {
    let matrix = input
        .lines()
        .collect::<Vec<&'a str>>();

    Grid { matrix }
}

fn main() {
    let input = include_str!("input.txt");
    let grid = parse_input(input);

    let num_pattern = regex::Regex::new(r"\d+").unwrap();

    let mut part1 = 0;

    for (line_i, &line) in grid.matrix.iter().enumerate() {
        for m in num_pattern.find_iter(line) {
            let number = m.as_str();

            if grid.check_symbols_in_bounds(line_i, m.start(), m.end() - 1) {
                let number = number.parse::<usize>().unwrap_or(0);
                println!("{}", number);
                part1 += number;
            }
        }
    }

    println!("Part 1: {}", part1);
}