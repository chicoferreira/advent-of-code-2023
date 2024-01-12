#[derive(PartialEq, Eq, Debug)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn checked_div(a: f64, b: f64) -> Option<f64> {
    if b == 0f64 {
        None
    } else {
        Some(a / b)
    }
}

fn calculate_mu(h1: &Hailstone, h2: &Hailstone) -> Option<f64> {
    let a_1 = h1.x as f64;
    let a_2 = h2.x as f64;
    let b_1 = h1.y as f64;
    let b_2 = h2.y as f64;
    let v_a_1 = h1.vx as f64;
    let v_a_2 = h2.vx as f64;
    let v_b_1 = h1.vy as f64;
    let v_b_2 = h2.vy as f64;
    // Assume that the slopes are different
    checked_div(b_2 * v_a_1 - b_1 * v_a_1 - a_2 * v_b_1 + a_1 * v_b_1, v_a_2 * v_b_1 - v_b_2 * v_a_1)
}

fn calculate_lambda(h1: &Hailstone, h2: &Hailstone) -> Option<f64> {
    let a_1 = h1.x as f64;
    let a_2 = h2.x as f64;
    let b_1 = h1.y as f64;
    let b_2 = h2.y as f64;
    let v_a_1 = h1.vx as f64;
    let v_a_2 = h2.vx as f64;
    let v_b_1 = h1.vy as f64;
    let v_b_2 = h2.vy as f64;
    // Assume that the slopes are different

    checked_div(b_1 * v_a_2 - b_2 * v_a_2 - a_1 * v_b_2 + a_2 * v_b_2, v_a_1 * v_b_2 - v_b_1 * v_a_2)
}

fn get_intersection_coordinate(x: i64, vx: i64, delta: f64) -> f64 {
    x as f64 + vx as f64 * delta
}

impl Hailstone {
    fn intersects_in_range(&self, other: &Hailstone, (start, end): (i64, i64)) -> bool {
        // println!("A: {:?}", self);
        // println!("B: {:?}", other);

        let (Some(mu), Some(lambda)) = (calculate_mu(self, other), calculate_lambda(self, other)) else {
            // if self.x == other.x && self.y == other.y {
            //     println!("lines are parallel and start at same point (true)");
            // } else {
            //     println!("lines are parralel start at different points (false)");
            // }
            return self.x == other.x && self.y == other.y;
        };

        if mu < 0f64 || lambda < 0f64 {
            // println!("lines intersect behind us lambda: {lambda}, mu: {mu}");
            return false;
        }

        let x_intersection = get_intersection_coordinate(self.x, self.vx, lambda);
        let y_intersection = get_intersection_coordinate(self.y, self.vy, lambda);

        if x_intersection >= start as f64 && x_intersection <= end as f64 &&
            y_intersection >= start as f64 && y_intersection <= end as f64 {
            // println!("Intersection inside range ({}, {})", x_intersection, y_intersection);
        } else {
            // println!("Intersection outside range ({}, {})", x_intersection, y_intersection);
        }

        return x_intersection >= start as f64 && x_intersection <= end as f64 &&
            y_intersection >= start as f64 && y_intersection <= end as f64;
    }
}

fn parse_hailstone(input: &str) -> Hailstone {
    let (positions, velocities) = input.split_once(" @ ").expect("input must contain @");
    let [x, y, z] = positions.splitn(3, ", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<i64>>()[..] else { panic!("input must contain 3 positions"); };

    let [vx, vy, vz] = velocities.splitn(3, ", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<i64>>()[..] else {
        panic!("input must contain 3 velocities");
    };
    Hailstone { x, y, z, vx, vy, vz }
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    input.lines().map(parse_hailstone).collect()
}

fn part1(hailstones: &[Hailstone], start: i64, end: i64) -> usize {
    let mut result = 0;
    for (i, a) in hailstones.iter().enumerate() {
        for b in hailstones[i + 1..].iter() {
            if a.intersects_in_range(b, (start, end)) {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let input = include_str!("input.txt");
    let hailstones = parse_hailstones(input);
    println!("Part 1: {}", part1(&hailstones, 200000000000000, 400000000000000));
}
