use std::fs;

fn main() {
    let input_path = "input/day_5.txt";
    let almanac_string = fs::read_to_string(input_path).unwrap();
    let almanac = Almanac::from_str(&almanac_string);

    let result = almanac.get_closest_location();
    println!(
        "The closest location to plant a seed - without seed ranges - is: {}",
        result
    );

    let almanac_with_seed_ranges = almanac.to_seed_range();

    let result = almanac_with_seed_ranges.get_closest_location();
    println!(
        "The closest location to plant a seed - with seed ranges - is: {}",
        result
    )
}

#[derive(Clone)]
struct RangeMap {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl RangeMap {
    fn new(destination_start: usize, source_start: usize, length: usize) -> Self {
        RangeMap {
            destination_start,
            source_start,
            length,
        }
    }

    fn is_in_range(&self, source: usize) -> bool {
        source >= self.source_start && source < (self.source_start + self.length)
    }

    fn get_map_destination(&self, source: usize) -> Option<usize> {
        match self.is_in_range(source) {
            true => Some(source - self.source_start + self.destination_start),
            false => None,
        }
    }
}

#[derive(Clone)]
struct AlmanacMap {
    range_maps: Vec<RangeMap>,
}

impl AlmanacMap {
    fn new(range_maps: Vec<RangeMap>) -> Self {
        AlmanacMap { range_maps }
    }

    fn get_destination(&self, source: usize) -> usize {
        let destination_from_map = self
            .range_maps
            .iter()
            .find_map(|r| r.get_map_destination(source));

        match destination_from_map {
            Some(destination) => destination,
            None => source,
        }
    }
}

#[derive(Clone)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn from_str(string: &str) -> Self {
        let mut iter = string.split("\n\n");
        let seeds_line = iter.next().unwrap();
        let seeds = seeds_line
            .strip_prefix("seeds:")
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let maps: Vec<AlmanacMap> = iter
            .map(|s| {
                let range_maps = s
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let range_params: Vec<usize> = line
                            .split_whitespace()
                            .take(3)
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect();
                        RangeMap::new(range_params[0], range_params[1], range_params[2])
                    })
                    .collect();
                AlmanacMap::new(range_maps)
            })
            .collect();

        Almanac { seeds, maps }
    }

    // This is terrible and will generate a vec of usize far too big to handle.
    fn to_seed_range(&self) -> Almanac {
        let seeds = self
            .seeds
            .chunks(2)
            .flat_map(|chunk| {
                let start = chunk[0];
                let range = chunk[1];

                start..start + range
            })
            .collect();

        Almanac {
            seeds,
            maps: self.maps.to_vec(),
        }
    }

    fn get_seed_location(&self, seed: usize) -> usize {
        self.maps
            .iter()
            .fold(seed, |source, map| map.get_destination(source))
    }

    fn get_closest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.get_seed_location(*seed))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_finds_the_correct_destination() {
        let map = AlmanacMap::new(vec![RangeMap::new(50, 98, 2), RangeMap::new(52, 50, 48)]);

        assert_eq!(map.get_destination(79), 81);
        assert_eq!(map.get_destination(14), 14);
        assert_eq!(map.get_destination(55), 57);
        assert_eq!(map.get_destination(13), 13);
    }

    #[test]
    fn it_finds_the_closest_location() {
        let almanac_str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let almanac = Almanac::from_str(almanac_str);
        let result = almanac.get_closest_location();

        assert_eq!(result, 35);

        let almanac_with_seed_range = almanac.to_seed_range();
        let result = almanac_with_seed_range.get_closest_location();

        assert_eq!(result, 46);
    }
}
