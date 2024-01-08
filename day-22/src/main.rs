#[derive(Debug, Eq, PartialEq)]
struct Brick {
    start_x: usize,
    start_y: usize,
    start_z: usize,
    end_x: usize,
    end_y: usize,
    end_z: usize,
}

struct Map(Vec<Brick>);

fn fall_down(bricks: &Vec<Brick>, ignore_index: Option<usize>) -> Vec<(Brick, bool)> { // (brick, has_fallen)
    let mut current_z_max = vec![vec![0; 10]; 10]; // the map is 10x10

    let mut result: Vec<(Brick, bool)> = Vec::new();
    for (index, brick) in bricks.iter().enumerate() {
        if Some(index) == ignore_index {
            continue;
        }

        let mut max_z = 0;

        for x in brick.start_x..=brick.end_x {
            for y in brick.start_y..=brick.end_y {
                if current_z_max[x][y] > max_z {
                    max_z = current_z_max[x][y];
                }
            }
        }

        let new_brick = Brick {
            start_x: brick.start_x,
            start_y: brick.start_y,
            start_z: max_z + 1,
            end_x: brick.end_x,
            end_y: brick.end_y,
            end_z: max_z + 1 + (brick.end_z - brick.start_z),
        };

        for x in new_brick.start_x..=new_brick.end_x {
            for y in new_brick.start_y..=new_brick.end_y {
                current_z_max[x][y] = new_brick.end_z;
            }
        }

        let has_fallen = brick.start_z != new_brick.start_z;

        result.push((new_brick, has_fallen));
    }

    result
}

fn part_1(map: &Map) -> usize {
    let fall_down_result = fall_down(&map.0, None);
    let bricks = fall_down_result.into_iter().map(|(brick, _)| brick).collect::<Vec<Brick>>();

    let mut result = 0;

    for index in 0..bricks.len() {
        if fall_down(&bricks, Some(index)).iter().all(|(_, has_fallen)| !has_fallen) {
            result += 1;
        }
    }

    result
}

fn part_2(map: &Map) -> usize {
    let fall_down_result = fall_down(&map.0, None);
    let bricks = fall_down_result.into_iter().map(|(brick, _)| brick).collect::<Vec<Brick>>();

    let mut result = 0;

    for index in 0..bricks.len() {
        result += fall_down(&bricks, Some(index)).iter().filter(|(_, has_fallen)| *has_fallen).count()
    }

    result
}

fn parse(input: &str) -> Map {
    fn parse_coords(input: &str) -> (usize, usize, usize) {
        let [x, y, z] = input
            .splitn(3, ',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..] else {
            panic!()
        };
        (x, y, z)
    }

    let mut bricks: Vec<Brick> = input
        .lines()
        .filter_map(|s| s.split_once('~'))
        .map(|(start, end)| (parse_coords(start), parse_coords(end)))
        .map(|((start_x, start_y, start_z), (end_x, end_y, end_z))|
            Brick { start_x, start_y, start_z, end_x, end_y, end_z })
        .collect();

    bricks.sort_by(|a, b| a.start_z.cmp(&b.start_z));

    Map(bricks)
}

fn main() {
    let input = include_str!("input.txt");
    let map = parse(input);
    let part1 = part_1(&map);
    println!("Part 1: {part1}");
    let part2 = part_2(&map);
    println!("Part 2: {part2}");
}
