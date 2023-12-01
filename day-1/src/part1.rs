fn main() {
    let input = include_str!("./input1.txt");

    let result = solution2(input);

    println!("{:?}", result);
}

#[warn(dead_code)]
fn solution1(input: &str) -> u32 {
    input.lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
        .map(|line: String| format!("{}{}", line.chars().next().unwrap(), line.chars().last().unwrap()))
        .map(|line| line.parse::<u32>().unwrap())
        .sum::<u32>()
}

fn solution2(input: &str) -> u32 {
    input.lines()
        .filter_map(|line| {
            let string: String = line.matches(char::is_numeric).collect();
            let x = string.chars().next()?.to_digit(10)?;
            let y = string.chars().last()?.to_digit(10)?;
            Some(x * 10 + y)
        })
        .sum::<u32>()
}
