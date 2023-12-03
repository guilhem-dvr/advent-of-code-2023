use std::fs;

use regex::Regex;

fn main() {
    let input_path = "input/day_3.txt";
    let schema = fs::read_to_string(input_path).unwrap();
    let engine_schematic = parse_engine_schematics(&schema);
    let result = sum_part_numbers(&engine_schematic);

    println!("The sum of part numbers is: {}", result);

    let result = sum_gear_ratios(&engine_schematic);

    println!("The sum of gear ratios is: {}", result)
}

#[derive(Debug, PartialEq)]
struct Symbol {
    row: usize,
    col: usize,
    is_potential_gear: bool,
}

impl Symbol {
    fn new(row: usize, col: usize, is_potential_gear: bool) -> Self {
        Symbol {
            row,
            col,
            is_potential_gear,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Number {
    row: usize,
    start: usize,
    end: usize,
    value: usize,
}

impl Number {
    fn new(row: usize, start: usize, end: usize, value: usize) -> Self {
        Number {
            row,
            start,
            end,
            value,
        }
    }

    fn is_part_number(&self, symbol: &Symbol) -> bool {
        match self.row.abs_diff(symbol.row) {
            0 => self.start == symbol.col + 1 || self.end == symbol.col - 1,
            1 => self.start <= symbol.col + 1 && self.end >= symbol.col - 1,
            _ => false,
        }
    }
}

fn parse_engine_schematics(schema: &str) -> (Vec<Symbol>, Vec<Number>) {
    let re = Regex::new(r"(?<number>\d+)|(?<symbol>[^\d\.])").unwrap();

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    schema.lines().enumerate().for_each(|(row, line)| {
        re.captures_iter(line).for_each(|cap| {
            if let Some(mat) = cap.name("symbol") {
                let is_potential_gear = matches!(mat.as_str(), "*");
                symbols.push(Symbol::new(row, mat.start(), is_potential_gear))
            }
            if let Some(mat) = cap.name("number") {
                let value = cap["number"].parse().unwrap();
                numbers.push(Number::new(row, mat.start(), mat.end() - 1, value))
            }
        })
    });

    (symbols, numbers)
}

fn sum_part_numbers((symbols, numbers): &(Vec<Symbol>, Vec<Number>)) -> usize {
    symbols
        .iter()
        .map(|s| {
            numbers
                .iter()
                // not super efficient...
                .filter(|n| n.is_part_number(s))
                .map(|n| n.value)
                .sum::<usize>()
        })
        .sum()
}

fn sum_gear_ratios((symbols, numbers): &(Vec<Symbol>, Vec<Number>)) -> usize {
    symbols
        .iter()
        .filter(|s| s.is_potential_gear)
        .map(|s| {
            numbers
                .iter()
                .filter(|n| n.is_part_number(s))
                .collect::<Vec<&Number>>()
        })
        .filter(|potential_gears| potential_gears.len() == 2)
        .map(|gears| gears[0].value * gears[1].value)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_engine_schematic() {
        let schema = "\
467..114#.
...*......";

        let expected_symbols = vec![Symbol::new(0, 8, false), Symbol::new(1, 3, true)];
        let expected_numbers = vec![Number::new(0, 0, 2, 467), Number::new(0, 5, 7, 114)];

        let result = parse_engine_schematics(schema);
        assert_eq!(result, (expected_symbols, expected_numbers))
    }

    #[test]
    fn it_sums_part_numbers() {
        let schema = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let engine_schematic = parse_engine_schematics(schema);

        let result = sum_part_numbers(&engine_schematic);
        assert_eq!(result, 4361);
    }

    #[test]
    fn it_sums_gear_ratios() {
        let schema = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let engine_schematic = parse_engine_schematics(schema);

        let result = sum_gear_ratios(&engine_schematic);
        assert_eq!(result, 467835);
    }
}
