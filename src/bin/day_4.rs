use std::{cmp, fs};

use regex::Regex;

fn main() {
    let input_path = "input/day_4.txt";
    let card_records = fs::read_to_string(input_path).unwrap();
    let cards = parse_cards(&card_records);
    let result = sum_points(&cards);

    println!("The sum of winning cards points is: {}", result)
}

#[derive(Debug, PartialEq)]
struct Card {
    winning_numbers: Vec<usize>,
    chosen_numbers: Vec<usize>,
    total: usize,
}

impl Card {
    fn new(winning_numbers: Vec<usize>, chosen_numbers: Vec<usize>) -> Self {
        Card {
            winning_numbers,
            chosen_numbers,
            total: 1,
        }
    }

    fn count_matches(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|w| self.chosen_numbers.contains(w))
            .count()
    }

    fn compute_points(&self) -> usize {
        let nb_correct: u32 = self.count_matches().try_into().unwrap();

        match nb_correct {
            0 => 0,
            n => 2_usize.pow(n - 1),
        }
    }
}

fn parse_cards(cards: &str) -> Vec<Card> {
    let re = Regex::new(r"^.*:(?<winnings>((\s+\d+)+)) \|(?<chosen>((\s+\d+)+))$").unwrap();
    cards
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();

            let winnings = cap.name("winnings").unwrap().as_str();
            let chosen = cap.name("chosen").unwrap().as_str();

            let winnings: Vec<usize> = winnings
                .split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();

            let chosen: Vec<usize> = chosen
                .split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();

            Card::new(winnings, chosen)
        })
        .collect()
}

fn sum_points(cards: &[Card]) -> usize {
    cards.iter().map(|c| c.compute_points()).sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_parses_cards() {
        let cards = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let expected = vec![Card::new(
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        )];
        let result = parse_cards(cards);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_compute_card_points() {
        let card = Card::new(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]);

        assert_eq!(card.compute_points(), 8);
    }

    #[test]
    fn it_sums_points() {
        let cards = vec![
            Card::new(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
            Card::new(
                vec![13, 32, 20, 16, 61],
                vec![61, 30, 68, 82, 17, 32, 24, 19],
            ),
        ];

        assert_eq!(sum_points(&cards), 10)
    }
}
