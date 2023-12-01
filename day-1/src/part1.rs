fn main() {
    let input = include_str!("./input1.txt");

    let result = input.lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
        .map(|line: String| format!("{}{}", line.chars().next().unwrap(), line.chars().last().unwrap()))
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();

    println!("{:?}", result);
}
