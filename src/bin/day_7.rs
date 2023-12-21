use std::{cmp::Ordering, collections::HashMap, fs, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let input_path = "input/day_7.txt";
    let hands_str = fs::read_to_string(input_path).unwrap();

    let mut hands: Vec<Hand> = hands_str
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect();

    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank + 1) * hand.bid);

    println!("The winnings for all hands is: {}", result)
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: [usize; 5]) -> Self {
        let mut frequency: HashMap<usize, usize> = HashMap::new();
        for card in cards {
            frequency
                .entry(card)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }

        match frequency.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if frequency.values().any(|v| *v == 4) {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if frequency.values().any(|v| *v == 3) {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("Somehow your hands is not a valid one."),
        }
    }

    fn as_rank(&self) -> usize {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_rank().partial_cmp(&other.as_rank())
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_rank().cmp(&other.as_rank())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [usize; 5],
    hand_type: HandType,
    bid: usize,
}

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<cards>[AKQJT98765432]{5})\s(?<bid>\d+)").unwrap());

        let caps = RE.captures(s).unwrap();

        let cards: [usize; 5] = caps
            .name("cards")
            .unwrap()
            .as_str()
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                n => n.to_string().parse().unwrap(),
            })
            .take(5)
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        let hand_type = HandType::from_cards(cards);

        let bid: usize = caps.name("bid").unwrap().as_str().parse().unwrap();

        Ok(Hand {
            cards,
            hand_type,
            bid,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ord => ord,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_hand_from_str() {
        let hand_str = "32T3K 765";

        let expected = Hand {
            cards: [3, 2, 10, 3, 13],
            hand_type: HandType::OnePair,
            bid: 765,
        };
        let result = Hand::from_str(hand_str).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_compares_hands() {
        let one_pair = Hand {
            cards: [3, 2, 10, 3, 13],
            hand_type: HandType::OnePair,
            bid: 0,
        };
        let one_pair_eq = Hand {
            cards: [3, 2, 10, 3, 13],
            hand_type: HandType::OnePair,
            bid: 0,
        };
        let two_pair_greater = Hand {
            cards: [13, 13, 6, 7, 7],
            hand_type: HandType::TwoPair,
            bid: 0,
        };
        let two_pair_lesser = Hand {
            cards: [13, 10, 11, 11, 10],
            hand_type: HandType::TwoPair,
            bid: 0,
        };

        assert_eq!(one_pair, one_pair_eq);
        assert!(two_pair_greater >= one_pair);
        assert!(two_pair_greater >= two_pair_lesser);

        assert!(two_pair_greater > two_pair_lesser);
    }

    #[test]
    fn it_does_everything_right() {
        let hands_str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut hands: Vec<Hand> = hands_str
            .lines()
            .map(|line| Hand::from_str(line).unwrap())
            .collect();
        hands.sort();
        let result = hands
            .iter()
            .enumerate()
            .fold(0, |acc, (rank, hand)| acc + (rank + 1) * hand.bid);

        assert_eq!(result, 6440)
    }
}
