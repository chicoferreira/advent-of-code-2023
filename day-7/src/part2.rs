use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card { J, N2, N3, N4, N5, N6, N7, N8, N9, T, Q, K, A }

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType { FiveOfAKind, FourOfAKind, FullHouse, ThreeOfAKind, TwoPair, OnePair, HighCard }

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand_type = self.get_hand_type();
        let other_hand_type = other.get_hand_type();
        if self_hand_type == other_hand_type {
            self.cards.cmp(&other.cards)
        } else {
            other_hand_type.cmp(&self_hand_type)
        }
    }
}

impl Hand {
    fn get_card_frequency(&self) -> Vec<(&Card, u64)> {
        self.cards.iter()
            .fold(vec![], |mut vec, card| {
                vec.iter_mut().find(|(c, _)| c == &card)
                    .map(|a| *a = (card, a.1 + 1))
                    .unwrap_or_else(|| vec.push((card, 1)));
                vec
            }).into_iter().collect()
    }

    fn get_hand_type(&self) -> HandType {
        let mut frequency = self.get_card_frequency();
        frequency.sort_by(|(_, c1), (_, c2)| (*c2).cmp(c1));

        let joker_frequency: u64 = frequency
            .iter()
            .find(|(card, _)| card == &&Card::J)
            .map(|(_, count)| *count)
            .unwrap_or(0);

        if joker_frequency == 5 {
            return HandType::FiveOfAKind;
        }

        frequency.retain(|(card, _)| card != &&Card::J);

        let card_most_count_expect_jokers = frequency
            .get(0)
            .map(|(_, count)| *count + joker_frequency);

        let card_second_most_count_expect_jokers = frequency
            .get(1)
            .map(|(_, count)| *count);

        match (card_most_count_expect_jokers, card_second_most_count_expect_jokers) {
            (Some(5), _) => HandType::FiveOfAKind,
            (Some(4), _) => HandType::FourOfAKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), _) => HandType::ThreeOfAKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

struct Game {
    hands: Vec<(Hand, u64)>,
}

fn main() {
    let input = include_str!("input.txt");
    let game = parser::parse_game(input);

    println!("Part 2: {}", part2(game));
}

fn part2(mut game: Game) -> u64 {
    game.hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));

    for x in &game.hands {
        println!("{:?}", x);
    }
    game.hands
        .iter()
        .zip(1..)
        .map(|((_, bid), rank)| rank * bid).sum()
}

mod parser {
    use crate::{Card, Game, Hand};

    fn parse_card(input: char) -> Card {
        match input {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card"),
        }
    }

    pub(crate) fn parse_hand(input: &str) -> Hand {
        let hands: [Card; 5] = input.chars().map(parse_card).collect::<Vec<Card>>().try_into().expect("hand has 5 cards");
        Hand { cards: hands }
    }

    fn parse_hand_and_bid(input: &str) -> (Hand, u64) {
        input.split_once(' ')
            .map(|(hand, bid)| (parse_hand(hand), bid.parse::<u64>().expect("bid is a number")))
            .expect("hand and bid are separated by a space")
    }

    pub(crate) fn parse_game(input: &str) -> Game {
        Game { hands: input.lines().map(parse_hand_and_bid).collect() }
    }
}

#[cfg(test)]
mod tests {
    use crate::HandType;
    use crate::parser::parse_hand;

    #[test]
    fn test_hand_type() {
        assert_eq!(parse_hand("KKJJJ").get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(parse_hand("JJJJJ").get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(parse_hand("KKKJJ").get_hand_type(), HandType::FiveOfAKind);
    }
}

