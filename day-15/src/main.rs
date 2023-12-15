fn get_hash_value(input: &str) -> u64 {
    input.chars()
        .fold(0, |acc, c| (acc + c as u64) * 17 % 256)
}

fn part2(input: &str) -> u64 {
    let mut hashmap: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];

    for part in input.split(',') {
        if let Some((lens, value)) = part.split_once('=') {
            let hash = get_hash_value(lens) as usize;
            let vec = &mut hashmap[hash];
            let value = value.parse().expect("string after = is number");

            if let Some((_, v)) = vec.iter_mut().find(|(l, _)| l == &lens) {
                *v = value;
            } else {
                vec.push((lens, value));
            }
        } else if let Some((lens, "")) = part.split_once('-') {
            let hash = get_hash_value(lens) as usize;
            hashmap[hash].retain(|(l, _)| l != &lens);
        } else {
            panic!("Invalid part: {}", part);
        }
    }

    hashmap.iter().enumerate().map(|(k, lens)| {
        let box_pos = (k + 1) as u64;
        lens.iter().enumerate().map(|(slot, (_, focal_length))| {
            let slot = slot as u64 + 1;
            box_pos * slot * focal_length
        }).sum::<u64>()
    }).sum()
}

fn main() {
    let input = include_str!("input.txt");

    let part1: u64 = input.split(',').map(get_hash_value).sum();
    println!("Part 1: {}", part1);

    let part2 = part2(input);
    println!("Part 2: {}", part2);
}
