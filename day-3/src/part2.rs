struct Grid<'a> {
    matrix: Vec<&'a str>,
}

impl<'a> Grid<'a> {
    fn get_line(&self, line: usize) -> Option<&'a str> {
        self.matrix.get(line).map(|s| *s)
    }

    fn get_numbers_around(&self, x: usize, y: usize, num_regex: &regex::Regex) -> Vec<u32> {
        fn sub_no_overflow(a: usize, b: usize) -> usize {
            if a > b {
                a - b
            } else {
                0
            }
        }

        [self.get_line(y - 1), self.get_line(y), self.get_line(y + 1)]
            .iter()
            .filter_map(|option| *option)
            .flat_map(|line| {
                num_regex.find_iter(line).filter(|m| {
                    let start = m.start();
                    let end = m.end() - 1;

                    (sub_no_overflow(start, 1) <= x && x <= end) ||
                        (start <= x && end + 1 >= x)
                }).map(|m| m.as_str()).collect::<Vec<&str>>()
            })
            .filter_map(|n: &str| n.parse::<u32>().ok())
            .collect()
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

    let num_regex: regex::Regex = regex::Regex::new(r"\d+").unwrap();

    let part2: u32 = grid.matrix.iter().enumerate()
        .flat_map(|(y, &line)| {
            line.match_indices('*')
                .map(|(x, _)| grid.get_numbers_around(x, y, &num_regex))
                .filter(|numbers| numbers.len() == 2)
                .map(|numbers| numbers.iter().product::<u32>())
                .collect::<Vec<u32>>()
        }).sum();

    println!("Part 2: {}", part2);
}