// Flip Flop %:
// starts off
// if receives low pulse, toggles on/off
// if it was off, send high pulse, if it was on, send low pulse
// if receives high pulse, ignores

// Conjunction &:
// starts with low pulse
// if remembers high pulses for all inputs, send low pulse
// if any input is low, send high pulse

// broadcaster:
// repeats the signal to all destinations

// button:
// send low pulse to broadcaster module

use std::collections::{HashMap, VecDeque};
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<usize, Pulse>),
    Output,
}

#[derive(Clone, Debug)]
struct Module {
    connected_to: Vec<usize>,
    module_type: ModuleType,
}

impl Module {
    fn pulse(&mut self, pulse: Pulse, from_module: usize) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::Broadcaster => Some(pulse),
            ModuleType::FlipFlop(state) => {
                match pulse {
                    Pulse::Low => {
                        *state = !*state;
                        Some(if *state { Pulse::High } else { Pulse::Low })
                    }
                    Pulse::High => {
                        None
                    }
                }
            }
            ModuleType::Conjunction(state) => {
                state.insert(from_module, pulse);
                if state.values().all(|&pulse| pulse == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleType::Output => None,
        }
    }
}

#[derive(Clone)]
struct ModuleConfiguration {
    broadcaster_id: usize,
    modules: Vec<Module>,
}

impl ModuleConfiguration {
    fn press_button(&mut self) -> (u64, u64) {
        let mut high = 0;
        let mut low = 0;
        let mut stack = VecDeque::new();
        stack.push_back((self.broadcaster_id, Pulse::Low, self.broadcaster_id));

        while let Some((module_id, pulse, from_id)) = stack.pop_front() {
            let module = &mut self.modules[module_id];

            if pulse == Pulse::High {
                high += 1;
            } else {
                low += 1;
            }

            if let Some(pulse) = module.pulse(pulse, from_id) {
                for connected_to_id in &module.connected_to {
                    stack.push_back((*connected_to_id, pulse, module_id));
                }
            }
        }

        (high, low)
    }
}

fn part1(mut module_configuration: ModuleConfiguration) -> u64 {
    let (mut high, mut low) = (0, 0);
    for _ in 0..1000 {
        let (new_high, new_low) = module_configuration.press_button();
        high += new_high;
        low += new_low;
    }

    high * low
}

fn lcm(x: u64, y: u64) -> u64 {
    let max = u64::max(x, y);
    let min = u64::min(x, y);
    let mut lcm = max;
    while lcm % min != 0 {
        lcm += max;
    }
    return lcm;
}

fn lcm_vec(vec: &[u64]) -> u64 {
    vec.iter().fold(1, |acc, x| lcm(acc, *x))
}

fn binary_counter_group(start: usize, module_configuration: &ModuleConfiguration) -> Vec<usize> {
    let mut group = vec![];
    let mut stack = vec![start];

    while let Some(module_id) = stack.pop() {
        let module = &module_configuration.modules[module_id];
        if let ModuleType::FlipFlop(_) = &module.module_type {
            group.push(module_id);
            for connected_to_id in &module.connected_to {
                stack.push(*connected_to_id);
            }
        }
    }

    group
}

fn get_binary_number(group: &[usize], module_configuration: &ModuleConfiguration) -> u64 {
    let mut number = 0;
    let mut mask = 1;
    for &module_id in group {
        let module = &module_configuration.modules[module_id];

        let is_connected_to_conjunction = module.connected_to.iter().any(|&connected_to|
            matches!(module_configuration.modules[connected_to].module_type, ModuleType::Conjunction(_))
        );

        if is_connected_to_conjunction {
            number |= mask;
        }

        mask <<= 1;
    }
    number
}

fn get_numbers_from_binary_counter_group(module_configuration: &ModuleConfiguration) -> Vec<u64> {
    let module = &module_configuration.modules[module_configuration.broadcaster_id];
    module.connected_to
        .iter()
        .map(|connected_to_index| binary_counter_group(*connected_to_index, module_configuration))
        .map(|group| get_binary_number(&group, module_configuration))
        .collect()
}


// Part 2 based on: https://github.com/ash42/adventofcode/blob/main/adventofcode2023/src/nl/michielgraat/adventofcode2023/day20/Day20.java
fn part2(module_configuration: ModuleConfiguration) -> u64 {
    lcm_vec(&get_numbers_from_binary_counter_group(&module_configuration))
}

fn main() {
    let input = include_str!("input.txt");
    let module_configuration = parse(input);

    let instant = Instant::now();
    let part1 = part1(module_configuration.clone());
    println!("Part 1: {} in {:?}", part1, instant.elapsed());

    let instant = Instant::now();
    let part2 = part2(module_configuration);
    println!("Part 2: {} in {:?}", part2, instant.elapsed());
}

fn parse(input: &'static str) -> ModuleConfiguration {
    let mut modules_from_name = HashMap::new();
    let mut modules_from_id = HashMap::new();

    let mut current_id = 0;
    for line in input.lines() {
        if let Some((mut name, connected_to)) = line.split_once(" -> ") {
            let module_type = match (&name[0..1], &name[1..]) {
                ("b", "roadcaster") => {
                    ModuleType::Broadcaster
                }
                ("%", rest) => {
                    name = rest;
                    ModuleType::FlipFlop(false)
                }
                ("&", rest) => {
                    name = rest;
                    ModuleType::Conjunction(HashMap::new())
                }
                _ => panic!("Unknown module type"),
            };

            let connected_to: Vec<&str> = connected_to.split(", ").collect();
            modules_from_name.insert(name, (current_id, connected_to, module_type));
            modules_from_id.insert(current_id, name);
            current_id += 1;
        }
    }

    // Conversion from string names to ids

    let mut modules = vec![];
    let mut broadcaster_id = 0;

    let mut output_modules = vec![];

    for id in 0..current_id {
        let module_name = modules_from_id[&id];
        let (id, connected_to_str, module_type) = modules_from_name[module_name].clone();

        let connected_to: Vec<usize> = connected_to_str.into_iter().map(|name| {
            if let Some(id) = modules_from_name.get(name).map(|(id, _, _)| *id) {
                id
            } else {
                let id = current_id;
                output_modules.push((id, name));
                modules_from_name.insert(name, (id, vec![], ModuleType::Output));
                modules_from_id.insert(current_id, name);
                current_id += 1;
                id
            }
        }).collect();

        if let ModuleType::Broadcaster = module_type {
            broadcaster_id = id;
        }

        modules.push(Module {
            connected_to,
            module_type,
        });
    }

    for _ in output_modules {
        modules.push(Module {
            connected_to: vec![],
            module_type: ModuleType::Output,
        });
    }

    let mut connections = vec![];

    for (current_id, module) in modules.iter().enumerate() {
        for id in &module.connected_to {
            connections.push((current_id, *id));
        }
    }

    for (from, to) in connections {
        if let ModuleType::Conjunction(state) = &mut modules[to].module_type {
            state.insert(from, Pulse::Low);
        }
    }

    ModuleConfiguration { broadcaster_id, modules }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let input = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let module_configuration = parse(input);
        assert_eq!(part1(module_configuration), 32000000);
    }

    #[test]
    fn test_example_input_2() {
        let input = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let module_configuration = parse(input);
        assert_eq!(part1(module_configuration), 11687500);
    }
}
