use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
struct GalaxyImage {
    galaxies: Vec<(usize, usize)>,
    expanded_rows: Vec<usize>,
    expanded_columns: Vec<usize>,
}

fn is_intersection(x1: usize, x2: usize, intersect: usize) -> bool {
    let min = usize::min(x1, x2);
    let max = usize::max(x1, x2);
    min <= intersect && intersect <= max
}

fn get_vertical_intersections(x1: usize, x2: usize, galaxy_image: &GalaxyImage) -> usize {
    galaxy_image.expanded_rows.iter().filter(|&&column| is_intersection(x1, x2, column)).count()
}

fn get_horizontal_intersections(y1: usize, y2: usize, galaxy_image: &GalaxyImage) -> usize {
    galaxy_image.expanded_columns.iter().filter(|&&row| is_intersection(y1, y2, row)).count()
}

fn distance(galaxy1: (usize, usize), galaxy2: (usize, usize), galaxy_image: &GalaxyImage, growing_factor: usize) -> usize {
    let (x1, y1) = galaxy1;
    let (x2, y2) = galaxy2;

    x2.abs_diff(x1) + y2.abs_diff(y1) + (get_horizontal_intersections(x1, x2, galaxy_image) + get_vertical_intersections(y1, y2, galaxy_image)) * (growing_factor - 1)
}

fn distances(galaxy_image: &GalaxyImage, growing_factor: usize) -> usize {
    galaxy_image.galaxies.iter().combinations(2).map(|galaxies| {
        distance(*galaxies[0], *galaxies[1], galaxy_image, growing_factor)
    }).sum()
}

fn parse(input: &str) -> GalaxyImage {
    let mut galaxies = Vec::new();
    let mut expanded_rows = Vec::new();
    let mut not_expanded_columns = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let mut expanded_row = true;
        for (x, character) in line.chars().enumerate() {
            if character == '#' {
                galaxies.push((x, y));
                not_expanded_columns.insert(x);
                expanded_row = false;
            }
        }
        if expanded_row {
            expanded_rows.push(y);
        }
    }

    let expanded_columns = (0..input.lines().next().map(&str::len).unwrap_or(0))
        .filter(|&column| !not_expanded_columns.contains(&column))
        .collect();

    GalaxyImage {
        galaxies,
        expanded_rows,
        expanded_columns,
    }
}

fn main() {
    let input = include_str!("input.txt");
    let galaxy_image = parse(input);

    println!("Part 1: {}", distances(&galaxy_image, 2));
    println!("Part 2: {}", distances(&galaxy_image, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let galaxies = [(3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)];
        let galaxy_image = GalaxyImage { galaxies: Vec::from_iter(galaxies), expanded_rows: vec![3, 7], expanded_columns: vec![2, 5, 8] };

        assert_eq!(distance(galaxies[0], galaxies[0], &galaxy_image, 2), 0);
        assert_eq!(distance(galaxies[0], galaxies[6], &galaxy_image, 2), 15);
        assert_eq!(distance(galaxies[2], galaxies[5], &galaxy_image, 2), 17);
        assert_eq!(distance(galaxies[7], galaxies[8], &galaxy_image, 2), 5);
        assert_eq!(distances(&galaxy_image, 2), 374);
        assert_eq!(distances(&galaxy_image, 10), 1030);
        assert_eq!(distances(&galaxy_image, 100), 8410);
    }
}