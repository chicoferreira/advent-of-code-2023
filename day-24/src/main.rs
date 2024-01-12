use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::identities::Zero;
use num_traits::ToPrimitive;

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

// From: https://github.com/TheAlgorithms/Rust/blob/master/src/math/gaussian_elimination.rs
fn gaussian_elimination(matrix: &mut [Vec<Ratio<BigInt>>]) -> Option<Vec<Ratio<BigInt>>> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }

    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    let mut result: Vec<Ratio<BigInt>> = vec![Ratio::zero(); size];
    for i in 0..size {
        if matrix[i][i] == Ratio::zero() {
            return None;
        }
        result[i] = matrix[i][size].clone() / matrix[i][i].clone();
    }
    Some(result)
}

fn echelon(matrix: &mut [Vec<Ratio<BigInt>>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == Ratio::zero() {} else {
        let factor = matrix[j + 1][i].clone() / matrix[i][i].clone();
        (i..size + 1).for_each(|k| {
            let ratio = matrix[i][k].clone();
            matrix[j + 1][k] -= factor.clone() * ratio;
        });
    }
}

fn eliminate(matrix: &mut [Vec<Ratio<BigInt>>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] == Ratio::zero() {} else {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i].clone() / matrix[i][i].clone();
            for k in (0..size + 1).rev() {
                let ratio = matrix[i][k].clone();
                matrix[j - 1][k] -= factor.clone() * ratio;
            }
        }
    }
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

fn part2(hailstones: &[Hailstone]) -> i64 {
    // Matrix derivation: https://typst.app/project/rsfD1FatoOPpvl9CdYHn1C
    let mut matrix = vec![vec![Ratio::zero(); 7]; 6];

    let h1 = &hailstones[1];
    let h2 = &hailstones[2];
    let h3 = &hailstones[4];

    let a1 = Ratio::from_integer(BigInt::from(h1.x));
    let b1 = Ratio::from_integer(BigInt::from(h1.y));
    let c1 = Ratio::from_integer(BigInt::from(h1.z));
    let a2 = Ratio::from_integer(BigInt::from(h2.x));
    let b2 = Ratio::from_integer(BigInt::from(h2.y));
    let c2 = Ratio::from_integer(BigInt::from(h2.z));
    let a3 = Ratio::from_integer(BigInt::from(h3.x));
    let b3 = Ratio::from_integer(BigInt::from(h3.y));
    let c3 = Ratio::from_integer(BigInt::from(h3.z));

    let va1 = Ratio::from_integer(BigInt::from(h1.vx));
    let va2 = Ratio::from_integer(BigInt::from(h2.vx));
    let va3 = Ratio::from_integer(BigInt::from(h3.vx));
    let vb1 = Ratio::from_integer(BigInt::from(h1.vy));
    let vb2 = Ratio::from_integer(BigInt::from(h2.vy));
    let vb3 = Ratio::from_integer(BigInt::from(h3.vy));
    let vc1 = Ratio::from_integer(BigInt::from(h1.vz));
    let vc2 = Ratio::from_integer(BigInt::from(h2.vz));
    let vc3 = Ratio::from_integer(BigInt::from(h3.vz));

    matrix[0][0] = &vb1 - &vb2;
    matrix[0][1] = &va2 - &va1;
    matrix[0][3] = &b2 - &b1;
    matrix[0][4] = &a1 - &a2;
    matrix[0][6] = &b2 * &va2 - &b1 * &va1 + &a1 * &vb1 - &a2 * &vb2;

    matrix[1][0] = &vb2 - &vb3;
    matrix[1][1] = &va3 - &va2;
    matrix[1][3] = &b3 - &b2;
    matrix[1][4] = &a2 - &a3;
    matrix[1][6] = &b3 * &va3 - &b2 * &va2 + &a2 * &vb2 - &a3 * &vb3;

    matrix[2][1] = &vc1 - &vc2;
    matrix[2][2] = &vb2 - &vb1;
    matrix[2][4] = &c2 - &c1;
    matrix[2][5] = &b1 - &b2;
    matrix[2][6] = &c2 * &vb2 - &c1 * &vb1 + &b1 * &vc1 - &b2 * &vc2;

    matrix[3][1] = &vc2 - &vc3;
    matrix[3][2] = &vb3 - &vb2;
    matrix[3][4] = &c3 - &c2;
    matrix[3][5] = &b2 - &b3;
    matrix[3][6] = &c3 * &vb3 - &c2 * &vb2 + &b2 * &vc2 - &b3 * &vc3;

    matrix[4][0] = &vc1 - &vc2;
    matrix[4][2] = &va2 - &va1;
    matrix[4][3] = &c2 - &c1;
    matrix[4][5] = &a1 - &a2;
    matrix[4][6] = &c2 * &va2 - &c1 * &va1 + &a1 * &vc1 - &a2 * &vc2;

    matrix[5][0] = &vc2 - &vc3;
    matrix[5][2] = &va3 - &va2;
    matrix[5][3] = &c3 - &c2;
    matrix[5][5] = &a2 - &a3;
    matrix[5][6] = &c3 * &va3 - &c2 * &va2 + &a2 * &vc2 - &a3 * &vc3;

    gaussian_elimination(&mut matrix).unwrap()[..3].iter().map(|ratio| ratio.to_integer().to_i64().unwrap()).sum()
}

fn main() {
    let input = include_str!("input.txt");
    let hailstones = parse_hailstones(input);
    println!("Part 1: {}", part1(&hailstones, 200000000000000, 400000000000000));

    println!("Part 2: {}", part2(&hailstones));
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
