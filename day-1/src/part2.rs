fn main() {
    let input = include_str!("./input1.txt");

    let mut mappings = vec![
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ];

    for i in 1..=9 {
        mappings.push((format!("{i}"), i));
    }

    let mut result = 0;

    for line in input.lines() {
        let mut digits = Vec::new();

        for i in 0..line.len() {
            for (s, n) in &mappings {
                if line[i..].starts_with(s) {
                    digits.push(n);
                }
            }
        }

        result += *digits.first().unwrap() * 10 + *digits.last().unwrap();
    }

    println!("{:?}", result);
}
