use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let input_path = "input/day_8.txt";
    let desert_map_str = fs::read_to_string(input_path).unwrap();
    let desert_map = DesertMap::from_str(&desert_map_str);

    let result = desert_map.find_the_exit();
    println!("The number of steps to exit the desert are: {}", result);
}

#[derive(Debug, PartialEq)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn new(left: &'a str, right: &'a str) -> Self {
        Node { left, right }
    }
}

#[derive(Debug, PartialEq)]
struct DesertMap<'a> {
    directions: Vec<char>,
    network: HashMap<&'a str, Node<'a>>,
}

impl<'a> DesertMap<'a> {
    fn new(directions: Vec<char>, network: HashMap<&'a str, Node<'a>>) -> Self {
        DesertMap {
            directions,
            network,
        }
    }

    fn from_str(string: &'a str) -> Self {
        let re_dir: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<directions>^[LR]+$)").unwrap());
        let re_net: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?<node>([A-Z]{3})) = \((?<left>([A-Z]{3})), (?<right>([A-Z]{3}))\)")
                .unwrap()
        });

        let mut lines = string.lines();

        let directions = re_dir
            .captures(lines.next().unwrap())
            .map(|caps| caps.name("directions").unwrap().as_str().chars().collect())
            .unwrap();

        lines.next().unwrap();

        let network = lines.fold(HashMap::new(), |mut map, line| {
            let caps = re_net.captures(line).unwrap();
            let node = caps.name("node").unwrap().as_str();
            let left = caps.name("left").unwrap().as_str();
            let right = caps.name("right").unwrap().as_str();

            map.insert(node, Node::new(left, right));

            map
        });

        DesertMap::new(directions, network)
    }

    fn find_the_exit(&self) -> usize {
        let mut current_node = self.network.get("AAA").unwrap();
        let mut nb_step = 0;

        for (step, direction) in self.directions.iter().cycle().enumerate() {
            let next_node_name = match direction {
                'L' => &current_node.left,
                'R' => &current_node.right,
                _ => panic!(),
            };
            current_node = self.network.get(next_node_name).unwrap();
            nb_step = step;

            if next_node_name == &"ZZZ" {
                break;
            }
        }

        nb_step + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_a_desert_map() {
        let map_str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let expected = DesertMap::new(
            vec!['L', 'L', 'R'],
            HashMap::from([
                ("AAA", Node::new("BBB", "BBB")),
                ("BBB", Node::new("AAA", "ZZZ")),
                ("ZZZ", Node::new("ZZZ", "ZZZ")),
            ]),
        );
        let result = DesertMap::from_str(map_str);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_finds_the_steps_to_exit() {
        let desert_map = DesertMap::new(
            vec!['L', 'L', 'R'],
            HashMap::from([
                ("AAA", Node::new("BBB", "BBB")),
                ("BBB", Node::new("AAA", "ZZZ")),
                ("ZZZ", Node::new("ZZZ", "ZZZ")),
            ]),
        );
        assert_eq!(desert_map.find_the_exit(), 6)
    }
}
