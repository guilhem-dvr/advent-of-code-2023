use std::fs;

use regex::Regex;

fn main() {
    let input_path = "input/day_2.txt";
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let game_records = fs::read_to_string(input_path).unwrap();
    let games = parse_game_records(&game_records);
    let result: usize = sum_possible_games(&games, &bag);

    println!("The sum of possible games is: {}", result);

    let result: usize = games.iter().map(|g| g.optimal_bag_power()).sum();

    println!("The sum of optimal bag powers is: {}", result);
}

struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Default, PartialEq)]
struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

impl Game {
    fn new(id: usize) -> Self {
        Game {
            id,
            ..Default::default()
        }
    }

    fn is_possible(&self, bag: &Bag) -> bool {
        self.max_red <= bag.red && self.max_green <= bag.green && self.max_blue <= bag.blue
    }

    fn optimal_bag_power(&self) -> usize {
        self.max_red * self.max_green * self.max_blue
    }
}

fn parse_game_records(records: &str) -> Vec<Game> {
    let re_game_id = Regex::new(r"^Game (?<id>\d+)").unwrap();
    let re_game_record =
        Regex::new(r"((?<red>\d+) red|(?<green>\d+) green|(?<blue>\d+) blue)+").unwrap();
    records
        .lines()
        .map(|l| {
            let (game_def, game_records) = l.split_once(':').unwrap();
            let id = re_game_id
                .captures(game_def)
                .unwrap()
                .name("id")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let mut game = Game::new(id);
            game_records.split(';').for_each(|game_record| {
                re_game_record.captures_iter(game_record).for_each(|c| {
                    if let Some(mat) = c.name("red") {
                        let red = mat.as_str().parse::<usize>().unwrap();
                        if red > game.max_red {
                            game.max_red = red
                        }
                    }
                    if let Some(mat) = c.name("green") {
                        let green = mat.as_str().parse::<usize>().unwrap();
                        if green > game.max_green {
                            game.max_green = green
                        }
                    }
                    if let Some(mat) = c.name("blue") {
                        let blue = mat.as_str().parse::<usize>().unwrap();
                        if blue > game.max_blue {
                            game.max_blue = blue
                        }
                    }
                });
            });
            game
        })
        .collect()
}

fn sum_possible_games(games: &[Game], bag: &Bag) -> usize {
    games
        .iter()
        .filter(|g| g.is_possible(bag))
        .map(|g| g.id)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_new_game_with_defaults() {
        assert_eq!(
            Game::new(1),
            Game {
                id: 1,
                max_red: 0,
                max_green: 0,
                max_blue: 0
            }
        );
    }

    #[test]
    fn it_validates_a_game_is_possible() {
        let bag = Bag {
            red: 2,
            green: 3,
            blue: 4,
        };
        let possible_game = Game {
            id: 1,
            max_red: 1,
            max_green: 2,
            max_blue: 2,
        };
        let impossible_game = Game {
            id: 2,
            max_red: 5,
            max_green: 3,
            max_blue: 2,
        };
        assert!(possible_game.is_possible(&bag));
        assert!(!impossible_game.is_possible(&bag));
    }

    #[test]
    fn it_parses_game_records() {
        let game_records = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game_records(game_records);
        assert_eq!(
            result,
            vec![Game {
                id: 1,
                max_red: 4,
                max_green: 2,
                max_blue: 6
            }]
        )
    }

    #[test]
    fn it_sums_possible_games() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        let games = vec![
            Game {
                id: 1,
                max_red: 4,
                max_green: 2,
                max_blue: 6,
            },
            Game {
                id: 2,
                max_red: 13,
                max_blue: 5,
                max_green: 6,
            },
            Game {
                id: 3,
                max_red: 1,
                max_blue: 3,
                max_green: 2,
            },
        ];
        let result = sum_possible_games(&games, &bag);
        assert_eq!(result, 4)
    }

    #[test]
    fn it_parses_and_sums_possible_games() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        let game_records = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = sum_possible_games(&parse_game_records(game_records), &bag);
        assert_eq!(result, 8);
    }
}
