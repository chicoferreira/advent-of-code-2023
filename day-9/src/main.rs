fn predict(slice: &[i64]) -> i64 {
    if slice.iter().all(|&x| x == 0) {
        return 0;
    }

    let prediction = predict(&slice.windows(2)
        .map(|chunks| chunks[1] - chunks[0])
        .collect::<Vec<i64>>());

    slice.last().expect("slice should not be empty") + prediction
}

fn predict_behind(slice: &[i64]) -> i64 {
    if slice.iter().all(|&x| x == 0) {
        return 0;
    }

    let prediction = predict_behind(&slice.windows(2)
        .map(|chunks| chunks[1] - chunks[0])
        .collect::<Vec<i64>>());

    slice.first().expect("slice should not be empty") - prediction
}

fn parse_slices(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line| line.split_whitespace()
            .map(|x| x.parse::<i64>().expect("input should be a number"))
            .collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>()
}

fn part1(slices: &Vec<Vec<i64>>) -> i64 {
    slices.iter()
        .map(|slice| predict(slice))
        .sum()
}

fn part2(slices: &Vec<Vec<i64>>) -> i64 {
    slices.iter()
        .map(|slice| predict_behind(slice))
        .sum()
}

fn main() {
    let input = include_str!("input.txt");

    let slices = parse_slices(input);
    println!("Part 1: {}", part1(&slices));
    println!("Part 2: {}", part2(&slices));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prediction() {
        assert_eq!(super::predict(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(super::predict(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_behind_prediction() {
        assert_eq!(super::predict_behind(&[10, 13, 16, 21, 30, 45]), 5);
    }
}
