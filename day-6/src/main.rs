struct Race {
    time: u64,
    distance: u64, // record
}

impl Race {
    fn get_number_of_ways_record_beaten(&self) -> u64 {
        let delta = ((self.time * self.time - 4 * self.distance) as f64).sqrt();
        let n1 = (self.time as f64 - delta) / 2f64;
        let n2 = (self.time as f64 + delta) / 2f64;

        (n2.ceil() - 1f64 - n1.floor()) as u64
    }
}

fn main() {
    let races = vec![
        Race { time: 40, distance: 215 },
        Race { time: 70, distance: 1051 },
        Race { time: 98, distance: 2147 },
        Race { time: 79, distance: 1005 },
    ];

    let part1: u64 = races.iter().map(Race::get_number_of_ways_record_beaten).product();
    println!("Part 1: {}", part1);

    let race = Race { time: 40709879, distance: 215105121471005 };
    println!("Part 2: {}", race.get_number_of_ways_record_beaten())
}

mod tests {
    use crate::Race;

    #[test]
    fn test_get_number_of_ways_record_beaten() {
        let race = Race { time: 7, distance: 9 };
        assert_eq!(race.get_number_of_ways_record_beaten(), 4);

        let race = Race { time: 15, distance: 40 };
        assert_eq!(race.get_number_of_ways_record_beaten(), 8);

        let race = Race { time: 30, distance: 200 };
        assert_eq!(race.get_number_of_ways_record_beaten(), 9);
    }
}
