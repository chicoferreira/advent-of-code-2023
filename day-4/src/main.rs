use std::collections::HashMap;

struct Card {
    winning_numbers: Vec<u8>,
    numbers_we_have: Vec<u8>,
}

impl Card {
    fn get_matching_numbers(&self) -> usize {
        self.numbers_we_have.iter().filter(|n| self.winning_numbers.contains(n)).count()
    }

    fn get_points(&self) -> usize {
        let winning_numbers = self.get_matching_numbers();
        match winning_numbers {
            0 => 0,
            _ => 2_usize.pow(winning_numbers.checked_sub(1).unwrap_or(0) as u32),
        }
    }
}

fn get_scratchcards(card_index: usize, card: &Card, cards: &[Card], memoization: &mut HashMap<usize, usize>) -> usize {
    if let Some(&scratchcards) = memoization.get(&card_index) {
        return scratchcards;
    }

    let mut scratchcards = 1; // We have the card itself

    for i in (card_index + 1)..=(card_index + card.get_matching_numbers()) {
        scratchcards += get_scratchcards(i, &cards[i], cards, memoization);
    }

    memoization.insert(card_index, scratchcards);
    scratchcards
}

fn part_2(cards: &[Card]) -> usize {
    let mut memoization = HashMap::new();

    cards.iter().enumerate().map(|(card_index, card)| get_scratchcards(card_index, card, cards, &mut memoization)).sum()
}

fn parse_numbers_str(numbers_str: &str) -> Vec<u8> {
    numbers_str.split_whitespace().filter_map(|n| n.parse::<u8>().ok()).collect()
}

fn parse_card_line(line: &str) -> Option<Card> {
    let (_, numbers_str) = line.split_once(':')?;
    let (winning_numbers_str, numbers_we_have_str) = numbers_str.split_once('|')?;

    Some(Card {
        winning_numbers: parse_numbers_str(winning_numbers_str),
        numbers_we_have: parse_numbers_str(numbers_we_have_str),
    })
}

fn main() {
    let input = include_str!("./input.txt");

    let cards = input.lines().filter_map(|line| parse_card_line(line)).collect::<Vec<_>>();

    let part_1 = cards.iter().map(|card| card.get_points()).sum::<usize>();
    let part_2 = part_2(&cards);

    println!("{:?}", part_1);
    println!("{:?}", part_2);
}
