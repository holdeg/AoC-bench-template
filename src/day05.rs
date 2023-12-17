use std::collections::HashSet;

use itertools::Itertools;

use crate::Solution;

pub struct PiecewiseShiftMap {
    mappings: HashSet<(u64, u64, i64)>,
}

impl FromIterator<(u64, u64, i64)> for PiecewiseShiftMap {
    fn from_iter<T: IntoIterator<Item = (u64, u64, i64)>>(iter: T) -> Self {
        Self {
            mappings: HashSet::from_iter(iter),
        }
    }
}

impl PiecewiseShiftMap {
    pub fn map(&self, source: u64) -> u64 {
        for (start, range, shift) in &self.mappings {
            if (start..&(start + range)).contains(&&source) {
                return (source as i64 + shift).try_into().unwrap();
            }
        }
        return source;
    }
}

#[derive(Clone, Debug)]
pub struct Day05;

impl Solution for Day05 {
    type ParsedInput = (HashSet<u64>, HashSet<u64>, Vec<PiecewiseShiftMap>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut lines = input_lines.lines();
        let seeds: HashSet<_> = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect();
        let mut seed_ranges = seeds.clone().into_iter();
        let mut seeds_2: HashSet<u64> = HashSet::default();
        while let (Some(start), Some(range)) = (seed_ranges.next(), seed_ranges.next()) {
            for seed in start..start + range {
                seeds_2.insert(seed);
            }
        }
        let maps = lines
            .group_by(|elt| *elt != "")
            .into_iter()
            .filter_map(|(empty, group)| if empty { Some(group) } else { None })
            .map(|mapping_def| {
                mapping_def
                    .skip(1)
                    .map(|line| {
                        let components = line
                            .split_whitespace()
                            .map(str::parse::<u64>)
                            .map(Result::unwrap)
                            .collect::<Vec<_>>();
                        (
                            components[1],
                            components[2],
                            components[0] as i64 - components[1] as i64,
                        )
                    })
                    .collect()
            })
            .collect();
        (seeds, seeds_2, maps)
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let (seeds, _, maps) = input;
        seeds
            .clone()
            .into_iter()
            .map(|mut seed| {
                for map in maps.iter() {
                    seed = map.map(seed);
                }
                seed
            })
            .fold(u64::MAX, |acc, val| acc.min(val))
            .to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let (_, seeds, maps) = input;
        seeds
            .clone()
            .into_iter()
            .map(|mut seed| {
                for map in maps.iter() {
                    seed = map.map(seed);
                }
                seed
            })
            .fold(u64::MAX, |acc, val| acc.min(val))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(
            Day05::solve_part_one(
                "seeds: 79 14 55 13

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
56 93 4"
            ),
            "35".to_string()
        )
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(Day05::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(Day05::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
