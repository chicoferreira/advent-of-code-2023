use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn count_possible_arrangements(states: &[SpringState], groups_of_damaged: &[usize]) -> usize {
    fn count_possible_arrangements_inner(
        cache: &mut HashMap<(Vec<SpringState>, Vec<usize>, usize), usize>,
        states: &[SpringState],
        groups: &[usize],
        accumulated_damage: usize,
    ) -> usize {
        if states.is_empty() {
            if groups.is_empty() {
                return 1;
            }

            if groups.len() == 1 && groups[0] == accumulated_damage {
                return 1;
            }

            return 0;
        }

        return match states[0] {
            SpringState::Operational => {
                if accumulated_damage == 0 {
                    return count_possible_arrangements_inner(cache, &states[1..], groups, accumulated_damage);
                }

                if groups.is_empty() || accumulated_damage != groups[0] {
                    return 0;
                }

                count_possible_arrangements_inner(cache, &states[1..], &groups[1..], 0)
            }

            SpringState::Damaged => {
                if groups.is_empty() || accumulated_damage + 1 > groups[0] {
                    return 0;
                }

                count_possible_arrangements_inner(cache, &states[1..], groups, accumulated_damage + 1)
            }

            SpringState::Unknown => {
                if let Some(answer) = cache.get(&(states.to_vec(), groups.to_vec(), accumulated_damage)).copied() {
                    return answer;
                }

                let mut arrangements = 0;

                if accumulated_damage == 0 {
                    arrangements += count_possible_arrangements_inner(cache, &states[1..], groups, accumulated_damage);
                }

                if !groups.is_empty() && accumulated_damage < groups[0] {
                    arrangements += count_possible_arrangements_inner(cache, &states[1..], groups, accumulated_damage + 1);
                }

                if !groups.is_empty() && accumulated_damage == groups[0] {
                    arrangements += count_possible_arrangements_inner(cache, &states[1..], &groups[1..], 0);
                }

                cache.insert((states.to_vec(), groups.to_vec(), accumulated_damage), arrangements);
                arrangements
            }
        };
    }
    count_possible_arrangements_inner(&mut HashMap::new(), states, groups_of_damaged, 0)
}

fn main() {
    let input = include_str!("input.txt");
    let input = parse_input(input);

    let instant = std::time::Instant::now();
    let part1 = input.iter().map(|(states, groups_of_damaged)| count_possible_arrangements(states, groups_of_damaged)).sum::<usize>();
    println!("Part 1: {} in {:?}", part1, instant.elapsed());

    let part2 = input.iter().map(|(states, groups_of_damaged)| count_possible_arrangements_5_times(states, &groups_of_damaged)).sum::<usize>();
    println!("Part 2: {} in {:?}", part2, instant.elapsed());
}

fn count_possible_arrangements_5_times(states: &[SpringState], groups_of_damaged: &[usize]) -> usize {
    let mut states = states.to_vec();
    states.push(SpringState::Unknown);
    states = states.repeat(5);
    states.pop();

    count_possible_arrangements(&states, &groups_of_damaged.repeat(5))
}

fn parse_input(input: &str) -> Vec<(Vec<SpringState>, Vec<usize>)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(input: &str) -> (Vec<SpringState>, Vec<usize>) {
    let option = input.split_once(' ').unwrap();
    let states = parse_spring_statuses(option.0);
    let groups_of_damaged = parse_groups_of_damaged(option.1);

    (states, groups_of_damaged)
}

fn parse_groups_of_damaged(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.trim().parse().unwrap()).collect()
}

fn parse_spring_statuses(input: &str) -> Vec<SpringState> {
    input.chars().map(|c| match c {
        '.' => SpringState::Operational,
        '#' => SpringState::Damaged,
        '?' => SpringState::Unknown,
        _ => panic!("Invalid spring status: {}", c),
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_state_valid() {
        assert_eq!(count_possible_arrangements(&parse_spring_statuses("???.###"), &[1, 1, 3]), 1);
        assert_eq!(count_possible_arrangements(&parse_spring_statuses(".??..??...?##."), &[1, 1, 3]), 4);
        assert_eq!(count_possible_arrangements(&parse_spring_statuses("?#?#?#?#?#?#?#?"), &[1, 3, 1, 6]), 1);
        assert_eq!(count_possible_arrangements(&parse_spring_statuses("????.#...#..."), &[4, 1, 1]), 1);
        assert_eq!(count_possible_arrangements(&parse_spring_statuses("????.######..#####."), &[1, 6, 5]), 4);
        assert_eq!(count_possible_arrangements(&parse_spring_statuses("?###????????"), &[3, 2, 1]), 10);
        assert_eq!(count_possible_arrangements_5_times(&parse_spring_statuses("???.###"), &[1, 1, 3]), 1);
    }
}
