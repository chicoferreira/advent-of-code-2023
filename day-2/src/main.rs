struct Cube {
    red: u32,
    blue: u32,
    green: u32,
}

struct Game {
    id: u32,
    cubes: Vec<Cube>,
}

fn parse_cube(input: &str) -> Cube {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for color in input.split(", ") {
        if let Some((number, color)) = color.split_once(' ') {
            match color {
                "red" => red += number.parse::<u32>().unwrap(),
                "blue" => blue += number.parse::<u32>().unwrap(),
                "green" => green += number.parse::<u32>().unwrap(),
                _ => (),
            }
        }
    }

    Cube { red, blue, green }
}

fn parse_game(input: &str) -> Option<Game> {
    let (game_str, cubes_str) = input.split_once(':')?;

    let id = game_str.split_once(' ')?.1.parse::<u32>().ok()?;

    let cubes = cubes_str
        .split(";")
        .map(|cube| parse_cube(cube.trim_start()))
        .collect();

    Some(Game { id, cubes })
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().filter_map(|line| parse_game(line)).collect()
}

fn part_1(games: &Vec<Game>) -> u32 {
    fn part_1_is_valid_game(game: &Game) -> bool {
        fn is_valid_cube(cube: &Cube) -> bool {
            cube.red <= 12 && cube.green <= 13 && cube.blue <= 14
        }

        game.cubes.iter().all(|cube| is_valid_cube(cube))
    }
    games.iter().filter(|game| part_1_is_valid_game(game)).map(|game| game.id).sum()
}

fn part_2(games: &Vec<Game>) -> u32 {
    fn get_max_cube_color(game: &Game, color_getter: fn(&Cube) -> u32) -> u32 {
        game.cubes.iter().map(color_getter).max().unwrap_or(0)
    }

    fn get_game_power(game: &Game) -> u32 {
        get_max_cube_color(game, |cube| cube.red)
            * get_max_cube_color(game, |cube| cube.blue)
            * get_max_cube_color(game, |cube| cube.green)
    }

    games.iter().map(|game| get_game_power(game)).sum()
}


fn main() {
    let input = include_str!("./input1.txt");
    let games = parse_games(input);

    println!("Part 1: {:?}", part_1(&games));
    println!("Part 2: {:?}", part_2(&games));
}
