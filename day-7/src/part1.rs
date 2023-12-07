use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card { N2, N3, N4, N5, N6, N7, N8, N9, T, J, Q, K, A }

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
            other.cards.cmp(&self.cards)
        } else {
            self_hand_type.cmp(&other_hand_type)
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
        frequency.sort_by(|a, b| b.1.cmp(&a.1));

        match (frequency[0].1, frequency.get(1).map(|(_, b)| b)) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, Some(2)) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, Some(2)) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
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

    println!("Part 1: {}", part1(&game));
}

fn part1(game: &Game) -> u64 {
    let mut x: Vec<&(Hand, u64)> = game.hands.iter().clone().collect();
    x.sort_by(|x, y| y.cmp(&x));

    x.iter().enumerate().map(|(rank, (_, bid))| (rank as u64 + 1) * bid).sum()
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

    fn parse_hand(input: &str) -> Hand {
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
    #[test]
    fn test_get_card_frequency() {
        use super::{Card, Hand};
        let hand = Hand { cards: [Card::N2, Card::N3, Card::N2, Card::N4, Card::N5] };
        let expected = vec![
            (&Card::N2, 2),
            (&Card::N3, 1),
            (&Card::N4, 1),
            (&Card::N5, 1),
        ];
        assert_eq!(hand.get_card_frequency(), expected);
    }

    #[test]
    fn test_get_hand_type() {
        use super::{Card, Hand, HandType};
        let hand = Hand { cards: [Card::N2, Card::N3, Card::N2, Card::N4, Card::N5] };
        assert_eq!(hand.get_hand_type(), HandType::OnePair);
    }

    #[test]
    fn compare_hands() {
        use super::{Card, Hand};
        let hand1 = Hand { cards: [Card::K, Card::K, Card::N6, Card::N7, Card::N7] };
        let hand2 = Hand { cards: [Card::K, Card::T, Card::J, Card::J, Card::T] };
        assert!(hand1 > hand2);
    }

    #[test]
    fn get_card_frequencies() {
        use super::{Card, Hand};
        let hand = Hand { cards: [Card::K, Card::K, Card::N6, Card::N7, Card::N7] };
        let expected = vec![
            (&Card::K, 2),
            (&Card::N6, 1),
            (&Card::N7, 2),
        ];
        assert_eq!(hand.get_card_frequency(), expected);
    }

    #[test]
    fn get_hand_type() {
        use super::{Card, Hand, HandType};
        let hand = Hand { cards: [Card::K, Card::K, Card::N6, Card::N7, Card::N7] };
        assert_eq!(hand.get_hand_type(), HandType::TwoPair);
    }
}

