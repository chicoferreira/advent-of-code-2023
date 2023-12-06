use std::ops::{Range, RangeBounds};
use std::time::Instant;

struct Mapping {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Mapping {
    fn get_mapping(&self, source: u64) -> Option<u64> {
        if source < self.source_start || source >= self.source_start + self.length {
            return None;
        }
        let offset = source - self.source_start;
        let destination = self.destination_start + offset;
        Some(destination)
    }

    fn get_source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.length
    }

    fn get_destination_range(&self) -> Range<u64> {
        self.destination_start..self.destination_start + self.length
    }
}

struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn get_mapping(&self, source: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(destination) = mapping.get_mapping(source) {
                return destination;
            }
        }

        source
    }

    fn get_range_mapping(&self, range: Range<u64>) -> Map {
        let mut mappings = Vec::new();
        for m in &self.mappings {
            if let Some(overlapping_range) = overlap(&m.get_source_range(), &range) {
                let source_start = overlapping_range.start;
                let destination_start = m.get_mapping(source_start).expect("overlap should be in range");
                mappings.push(Mapping {
                    destination_start,
                    source_start,
                    length: overlapping_range.end - overlapping_range.start,
                });
            }
        }

        Map { mappings }
    }

    // fn get_map_mapping(&self, map: Map) -> Map {
    //     let result = Vec::new();
    // }
}

fn overlap(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);

    if start > end {
        return None;
    }
    Some(start..end)
}

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

impl Almanac {
    fn get_location_number(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil_map.get_mapping(seed);
        let fertilizer = self.soil_to_fertilizer_map.get_mapping(soil);
        let water = self.fertilizer_to_water_map.get_mapping(fertilizer);
        let light = self.water_to_light_map.get_mapping(water);
        let temperature = self.light_to_temperature_map.get_mapping(light);
        let humidity = self.temperature_to_humidity_map.get_mapping(temperature);
        let location = self.humidity_to_location_map.get_mapping(humidity);

        location
    }
}

mod parser {
    use crate::{Almanac, Map, Mapping};

    fn parse_seeds(seeds_line: &str) -> Option<Vec<u64>> {
        Some(seeds_line.split_once(':')?.1
            .split_whitespace()
            .filter_map(|seed| seed.trim().parse::<u64>().ok())
            .collect())
    }

    fn parse_mapping(range_line: &str) -> Option<Mapping> {
        let mut n = range_line.splitn(3, ' ');

        let destination_start = n.next()?.parse::<u64>().ok()?;
        let source_start = n.next()?.parse::<u64>().ok()?;
        let length = n.next()?.parse::<u64>().ok()?;

        Some(Mapping {
            source_start,
            destination_start,
            length,
        })
    }

    fn parse_map(lines_iter: &mut std::str::Lines) -> Map {
        let mut mappings = Vec::new();
        for line in lines_iter.skip(1).take_while(|line| !line.is_empty()) {
            if let Some(mapping) = parse_mapping(line) {
                mappings.push(mapping);
            }
        }
        Map { mappings }
    }

    pub(crate) fn parse_seeds_and_almanac(input: &str) -> Almanac {
        let mut lines = input.lines();
        let seeds_line = &lines.next();
        let seeds = parse_seeds(seeds_line.unwrap()).unwrap();
        lines.next(); // Skip empty line

        let seed_to_soil_map = parse_map(&mut lines);
        let soil_to_fertilizer_map = parse_map(&mut lines);
        let fertilizer_to_water_map = parse_map(&mut lines);
        let water_to_light_map = parse_map(&mut lines);
        let light_to_temperature_map = parse_map(&mut lines);
        let temperature_to_humidity_map = parse_map(&mut lines);
        let humidity_to_location_map = parse_map(&mut lines);

        let almanac = Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        };

        almanac
    }
}

fn main() {
    let input = include_str!("input.txt");
    let almanac = parser::parse_seeds_and_almanac(input);

    let instant = Instant::now();
    let part1 = part1(&almanac).unwrap();
    println!("Part 1: {:?} ({:?})", part1, instant.elapsed());

    let instant = Instant::now();
    let part2 = part2(&almanac).unwrap();
    println!("Part 2: {:?} ({:?})", part2, instant.elapsed());
}

fn part2(almanac: &Almanac) -> Option<u64> {
    almanac.seeds.chunks(2).flat_map(|pair| {
        let start = pair[0];
        let length = pair[1];
        (start..start + length).map(|seed| almanac.get_location_number(seed)).min()
    }).min()
}

fn part1(almanac: &Almanac) -> Option<u64> {
    almanac.seeds.iter().map(|&seed| almanac.get_location_number(seed)).min()
}

#[cfg(test)]
mod tests {
    use super::overlap;
    use std::ops::Range;

    #[test]
    fn test_get_range_mapping() {
        let map = super::Map {
            mappings: vec![
                super::Mapping {
                    destination_start: 0,
                    source_start: 0,
                    length: 10,
                },
                super::Mapping {
                    destination_start: 10,
                    source_start: 10,
                    length: 10,
                },
                super::Mapping {
                    destination_start: 20,
                    source_start: 20,
                    length: 10,
                },
            ],
        };

        let range = 5..15;
        let result = map.get_range_mapping(range);
        assert_eq!(result.mappings.len(), 2);
        assert_eq!(result.mappings[0].destination_start, 5);
        assert_eq!(result.mappings[0].source_start, 5);
        assert_eq!(result.mappings[0].length, 5);
    }
}
