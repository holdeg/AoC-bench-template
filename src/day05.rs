use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::Solution;

#[derive(Default)]
pub struct PiecewiseShiftMap {
    /// Map of ranges to shifts / y-intercepts.
    /// Ranges are half-open [x, y) (not including the upper limit).
    /// E.g., the key-value pair (2, 5): 3 means
    /// 2 -> 5, 3 -> 6, 4 -> 7.     
    pieces: HashMap<[u64; 2], i64>,
}

impl FromIterator<([u64; 2], i64)> for PiecewiseShiftMap {
    fn from_iter<T: IntoIterator<Item = ([u64; 2], i64)>>(iter: T) -> Self {
        Self {
            pieces: HashMap::from_iter(iter),
        }
    }
}

impl PiecewiseShiftMap {
    pub fn map(&self, source: u64) -> u64 {
        for (range, shift) in self.pieces.iter() {
            if (range[0]..range[1]).contains(&&source) {
                return (source as i64 + shift).try_into().unwrap();
            }
        }
        return source;
    }

    /// Takes `f(x)` and produces `g(f(x))` (where `g(x)` is this
    /// function).
    pub fn compose(&self, f: &PiecewiseShiftMap) -> Self {
        // Consider
        //        |  0 <= x < 10 => x + 5
        // f(x) = { 10 <= x < 15 => x - 10
        //        |        other => x
        // and
        //        |  3 <= x < 5  => x - 2
        // g(x) = {  1 <= x < 3  => x + 2
        //        |        other => x
        self.pieces
            .keys()
            .flatten()
            .chain(f.pieces.keys().flatten()) // (0, 10, 10, 15, 1, 3, 3, 5)
            .unique() // (0, 10, 15, 1, 3, 5)
            .sorted() //(0, 1, 3, 5, 10, 15)
            .map(|endpoint| {
                (
                    endpoint,
                    self.map(f.map(*endpoint)) as i64 - *endpoint as i64,
                )
            })
            .tuple_windows()
            .map(|((start, shift), (end, _))| ([*start, *end], shift))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Day05;

impl Solution for Day05 {
    type ParsedInput = (HashSet<u64>, HashSet<u64>, PiecewiseShiftMap);

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
        // let mut seed_ranges = seeds.clone().into_iter();
        // let mut seeds_2: HashSet<u64> = HashSet::default();
        // while let (Some(start), Some(range)) = (seed_ranges.next(), seed_ranges.next()) {
        //     for seed in start..start + range {
        //         seeds_2.insert(seed);
        //     }
        // }
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
                        // Input looks like: dest_start source_start range
                        (
                            [components[1], components[1] + components[2]],
                            components[0] as i64 - components[1] as i64,
                        )
                    })
                    .collect()
            })
            .fold(PiecewiseShiftMap::default(), |acc, f: PiecewiseShiftMap| {
                f.compose(&acc)
            });
        (seeds.clone(), seeds.clone(), maps)
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let (seeds, _, composition) = input;
        seeds
            .clone()
            .into_iter()
            .map(|seed| composition.map(seed))
            .fold(u64::MAX, |acc, val| acc.min(val))
            .to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        0.to_string()
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
